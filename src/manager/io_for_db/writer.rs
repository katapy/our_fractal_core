
pub mod writer {

    use std::fs::{File, OpenOptions};
    use std::io::{Write};

    /// Write file.
    pub struct Writer {
        path: std::path::PathBuf,
        file: File,
    }

    impl Writer {
        /// Open file in write-read mode.
        /// This function panic when file does not exist.
        pub fn open (path : &std::path::PathBuf) -> Result<Writer, std::io::Error> {
            let f = OpenOptions::new()
                .read(true)
                .write(true)
                .create(true)
                .append(true)
                .open(path)?;
            Ok(Writer {
                path: path.to_path_buf(),
                file: f,
            })
        }

        /// Opens a file in write-only mode. 
        pub fn create (path : &std::path::PathBuf) -> Result<Writer, std::io::Error> {
            let f = OpenOptions::new()
                .write(true)
                .create(true)
                .truncate(true)
                .open(path)?;
            Ok(
                Writer {
                    path: path.to_path_buf(),
                    file: f,
                }
            )
        }

        /// write on file
        /// * `buf` - Written data on file.
        pub fn write(&mut self, buf: &[u8]) {
            self.file.write(buf).unwrap();
        }
    }
}
