
pub mod path_manager {
    use std::fs;
    use std::error;
    use std::path::PathBuf;

    // Change the alias to `Box<dyn error::Error>`.
    type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

    #[derive(Clone, Debug)]
    pub struct PathManager {
        pub dir_path: PathBuf,
        pub table_name: String,
        pub data_name: String,
    }

    impl PathManager {

        /// Create new  
        /// * `dir_path` - Directory path.
        /// * `table_name` - Data table name.
        /// * `data_name` - Data name.
        pub fn new(dir_path: PathBuf, table_name: String, data_name: String) -> PathManager {
            PathManager {
                dir_path: dir_path,
                table_name: table_name,
                data_name: data_name,
            }
        }

        /// Definition file path.
        pub fn get_def_path(&self) -> Result<PathBuf> {
            let path = &mut self.dir_path.clone();
            path.push(self.table_name.to_string());
            path.push("org");
            if !path.exists() {
                fs::create_dir_all(&path)?;
            }
            path.push("dbdf.mfd");
            Ok(path.to_path_buf())
        }

        /// Data file path.
        pub fn get_data_path(&self) -> Result<PathBuf> {
            let path = &mut self.dir_path.clone();
            path.push(self.table_name.to_string());
            path.push("org/data");
            if !path.exists() {
                fs::create_dir_all(&path)?;
            }
            path.push(format!("{}.mfd", self.data_name));
            Ok(path.to_path_buf())
        }
    }
}
