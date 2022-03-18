
pub mod reader;
pub mod writer;
pub mod path_manager;

pub mod io {

    const STX: u8 = 0x02;
    const ETX: u8 = 0x03;

    use crate::manager::io_for_db::writer::writer::Writer;
    use crate::manager::io_for_db::reader::reader::Reader;

    use std::convert::TryFrom;
    use std::error;
    use std::path::PathBuf;
    use std::str;

    // Change the alias to `Box<dyn error::Error>`.
    type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

    /// Read/Write mode
    #[derive(Debug)]
    pub enum Mode {
        Read,
        Write,
    }

    /// Binary data manager. It use for Read/Write file.
    pub struct BinaryManager {
        /// binary data.
        buf: Vec<u8>,
        /// index which reading now.
        index: usize,
        // child binary manager. its size have to 0 or 1.
        pub child: Vec<BinaryManager>
    }

    impl BinaryManager {
        /// Create new.
        pub fn new() -> BinaryManager {
            let buf: Vec<u8> = Vec::new();
            let child: Vec<BinaryManager> = Vec::new();
            BinaryManager{
                buf: buf,
                index: 0,
                child: child,
            }
        }

        /// create child.
        /// * `mode` - Read/Write.
        pub fn create_child(&mut self, mode: Mode) {
            let child_vec: Vec<BinaryManager> = Vec::new();
            let buf: Vec<u8> = match mode {
                Mode::Read => self.buf[self.index..].to_vec(),
                Mode::Write => Vec::new(),
            };
            let child = BinaryManager{
                buf: buf,
                index: 0,
                child: child_vec,
            };
            self.child = vec![child];
        }

        /// get child binary manager.
        pub fn get_child(&mut self) -> Result<&mut BinaryManager> {
            match self.child.iter_mut().find(|_| true) {
                Some(child) => Ok(child),
                None => Err("Child binary manager cannot found")?
            }
        }

        /// Exit the child manager and add the child data.
        pub fn end_child_and_add_data(&mut self) -> Result<()> {
            let end = self.get_child()?.add_end_data();
            self.buf.append(&mut end?);
            // Clear child.
            self.child = Vec::new();
            Ok(())
        }

        /// add u32 data.
        /// * `data` - append data.
        pub fn add_u32(&mut self, data: u32) -> Result<()> {
            let mut vec:Vec<u8> = Vec::new();
            u32_to_slice(data, &mut vec, 24)?;
            self.buf.append(&mut vec);
            Ok(())
        }

        /// add u8 data.
        /// * `data` - append data.
        pub fn add_u8(&mut self, data: u8){
            self.buf.push(data);
        }

        /// add bool data.
        /// * `data` - append data.
        pub fn add_bool(&mut self, data: bool){
            if data {
                self.buf.push(0x01)
            }
            else{
                self.buf.push(0x00)
            }
        }

        /// add string data.
        /// * `data` - append data.
        pub fn add_str(&mut self, data: &String) -> Result<()> {
            self.add_usize(data.len())?;
            for b in data.as_bytes(){
                self.buf.push(*b);
            }
            Ok(())
        }

        /// add usize data.
        /// * `data` - append data.
        pub fn add_usize(&mut self, data: usize) -> Result<()> {
            let data = data as u128;
            let mut vec:Vec<u8> = Vec::new();
            u128_to_slice(data, &mut vec, 120)?;
            self.buf.append(&mut vec);
            Ok(())
        }

        /// add start data(STX)
        pub fn add_start_data(&mut self) {
            self.buf.append(&mut vec![0x00, STX]);
        }

        /// add end data (EXT, check sum)
        pub fn add_end_data(&mut self) -> Result<Vec<u8>> {
            self.buf.push(ETX);
            let check_sum = self.check_sum();
            self.buf.push(check_sum?);
            Ok(self.buf.iter().as_slice().to_vec())
        }

        /// calculate check sum.
        /// * `data` - calculated data.
        pub fn check_sum(&mut self) -> Result<u8> {
            let mut sum: u16 = 0;
            for d in self.buf.iter_mut() {
                let mut _d: u16 = TryFrom::try_from(*d)?;
                // AND mask for avoid overflowing digits
                sum = (sum + _d) & 0xff;
            }
            let check: u8 = TryFrom::try_from(sum)?;
            Ok(check)
        }

        /// Write buffer data
        /// * `data` - file path
        pub fn write(&self, path: &PathBuf) -> Result<()> {
            let mut writer = Writer::create(path)?;
            writer.write(&self.buf);
            Ok(())
        }

        /// Read file data to end.
        /// * `path` - file path.
        pub fn read_to_end(&mut self, path: &PathBuf) -> Result<()> {
            let mut reader = Reader::open(path)?;
            reader.read_to_end()?;
            self.buf = reader.result;
            Ok(())
        }

        /// Proceed to the next STX.  
        /// Return false if STX is NOT exist or ETX found
        pub fn next_stx_index(&mut self) -> bool {
            loop {
                // could not found STX.
                if self.index >= self.buf.len() {
                    return false;
                }

                // return false when ETX found
                if self.buf[self.index] == ETX {
                    return false;
                }

                // Skip data to STX
                if self.buf[self.index] == STX {
                    break;
                }
                self.index += 1;
            }
            true
        }

        /// read check sum.  
        /// return index if check sum is Ok and 0 if check sum error.
        pub fn read_check_sum(&mut self) -> Result<usize> {
            let check_sum = self.buf[self.index] as u8;
            // get buf until check sum data.
            self.buf = self.buf[..self.index].to_vec();
            if check_sum == self.check_sum()? {
                Ok(self.index)
            }
            else {
                Err("Check sum error!")?
            }
        }

        /// read child data.  
        /// return true if child check sum is Ok and false if child check sum error.
        pub fn read_child(&mut self) -> Result<()> {
            self.index += self.get_child()?.read_check_sum()?;
            Ok(())
        }

        /// read u8 data.
        pub fn read_u8(&mut self) -> u8 {
            let u = self.buf[self.index];
            self.index += 1;
            u
        }

        /// read u32 data.
        pub fn read_u32(&mut self) -> Result<u32> {
            let mut data: u32 = 0;
            for (i, b) in self.buf[self.index..self.index + 4].iter().enumerate() {
                let _b: u32 = TryFrom::try_from(*b)?;
                data = data | (_b << ((3 - i) * 8) as u32);
            }
            self.index += 4;
            Ok(data)
        }

        /// read u128 data.
        pub fn read_u128(&mut self) -> Result<u128> {
            let mut data: u128 = 0;
            for (i, b) in self.buf[self.index..self.index + 16].iter().enumerate() {
                let _b: u128 = TryFrom::try_from(*b)?;
                data = data | (_b << ((15 - i) * 8) as u128);
            }
            self.index += 16;
            Ok(data)
        }

        /// read u8 vec data.
        /// * `len` - data length.
        pub fn read_u8_vec(&mut self, len: usize) -> Vec<u8> {
            let vec = &self.buf[self.index..self.index + len];
            self.index += len;
            vec.to_vec()
        }

        /// read string data.
        pub fn read_str(&mut self) -> Result<String> {
            // read string of length.
            let len: usize = self.read_u128()? as usize;
            // read string.
            let str = str::from_utf8(&self.buf[self.index .. self.index + len])?;
            self.index += len;
            Ok(str.to_string())
        }

        /// Read String data.  It use when read data value which type is String.
        pub fn read_str_value(&mut self) -> Result<Vec<u32>> {
            // read string of length.
            let len: usize = self.read_u128()? as usize;
            let mut vec: Vec<u32> = Vec::new();
            vec = u128_to_u32slice(len as u128, &mut vec, 96)?;
            // read string.
            for _ in 0..len {
                self.read_u32()?;
            }
            self.index += len;
            Ok(vec)
        }
    }

    /// Convert u32 to u8 slice
    /// * `data` - original data.
    /// * `buf` - result.
    /// * `tags` - digit. it must set 24 when use.
    fn u32_to_slice(data: u32, mut buf: &mut Vec<u8>, digit: u32) -> Result<Vec<u8>> {
        let a:u32 = data >> digit & 0xff;
        let a: u8 = TryFrom::try_from(a)?;
        buf.push(a);
        if digit >= 8{
            u32_to_slice(data, &mut buf, digit - 8)?;
        }
        Ok(buf.to_vec())
    }

    /// Convert u128 to u8 slice
    /// * `data` - original data.
    /// * `buf` - result.
    /// * `tags` - digit. it must set 120 when use.
    pub fn u128_to_slice(data: u128, mut buf: &mut Vec<u8>, digit: u128) -> Result<Vec<u8>> {
        let a:u128 = data >> digit & 0xff;
        let a: u8 = TryFrom::try_from(a)?;
        buf.push(a);
        if digit >= 8{
            u128_to_slice(data, &mut buf, digit - 8)?;
        }
        Ok(buf.to_vec())
    }

    /// Convert u128 to u32 slice
    /// * `data` - original data.
    /// * `buf` - result.
    /// * `tags` - digit. it must set 96 when use.
    fn u128_to_u32slice(data: u128, mut buf: &mut Vec<u32>, digit: u128) -> Result<Vec<u32>> {
        let a:u128 = data >> digit & 0xffffffff;
        let a: u32 = TryFrom::try_from(a)?;
        buf.push(a);
        if digit >= 32{
            u128_to_u32slice(data, &mut buf, digit - 32)?;
        }
        Ok(buf.to_vec())
    }

    #[cfg(test)]
    extern crate speculate;

    #[cfg(test)]
    use speculate::speculate;

    #[cfg(test)]
    speculate! {

        use std::{thread, time};
        use rstest::rstest;

        describe "module" {
            it "u32 to slice" {
                let tag: u32 = 0x2010_0010;
                let mut buf: Vec<u8> = Vec::new();
                u32_to_slice(tag, &mut buf, 24);
                assert_eq!(buf, vec![0x20, 0x10, 0x00, 0x10]);
            }

            it "u128 to u32slice" {
                let tag: u128 = 0x1010_0010_2010_0010_3010_0010_4010_0010;
                let mut buf: Vec<u32> = Vec::new();
                u128_to_u32slice(tag, &mut buf, 96);
                assert_eq!(buf, vec![0x1010_0010, 0x2010_0010, 0x3010_0010, 0x4010_0010]);
            }
        }
    }
}
