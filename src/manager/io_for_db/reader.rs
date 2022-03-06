
pub mod reader {

    use std::fs::File;
    use std::io::Read;
    use std::result::Result;

    /// Read file.
    pub struct Reader {
        file: File,
        pub result: Vec<u8>,
    }

    impl Reader {
        /// Open file.
        /// * `path` - File path.
        pub fn open(path: &std::path::PathBuf) -> Result<Reader, std::io::Error> {
            let file = File::open(path)?;
            let buf = Vec::new();
            Ok(
                Reader{
                    file: file,
                    result: buf,
                }
            )
        }

        /// Read file.
        #[warn(dead_code)]
        pub fn read(&mut self) -> Result<usize, std::io::Error> {
            self.result = Vec::new();
            self.file.read(&mut self.result)
        }

        /// Read file to end.
        pub fn read_to_end(&mut self) -> Result<usize, std::io::Error> {
            self.result = Vec::new();
            self.file.read_to_end(&mut self.result)
        }
    }
}
