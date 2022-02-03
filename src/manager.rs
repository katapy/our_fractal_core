
pub mod data;
pub mod io_for_db;

pub mod manager {
    
    use crate::manager::data::definition::definition::{Definition, Type};
    use crate::manager::data::data::{Data, DataRoot};
    use crate::manager::io_for_db::path_manager::path_manager::PathManager;
    use crate::manager::io_for_db::io::{BinaryManager, Mode, u128_to_slice};

    use std::error;
    use std::path::PathBuf;
    use std::sync::Mutex;
    use once_cell::sync::Lazy;
    use std::env;
    use dotenv::dotenv;

    type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

    pub static GLOBAL_DATA: Lazy<Mutex<Manager>> = Lazy::new(|| {
        dotenv().ok();
        let path = env::var("path").expect("path is not found in env");
        let table_name = env::var("table_name").expect("table_name is not found");
        let data_name = env::var("data_name").expect("data_name is not found");
        let path = &std::path::PathBuf::from(path);
        let manager = Manager::new(path, table_name, data_name);
        Mutex::new(manager)
    });

    /// get static manager
    pub async fn static_manager<'a>() -> std::sync::MutexGuard<'static, Manager> {
        GLOBAL_DATA.lock().unwrap()
    }

    /// OurFractal DB Manager.
    pub struct Manager {
        def_list: Vec<Definition>,
        parent_data: Data,
        path_manager: PathManager,
    }

    impl Manager {

        /// Create new manager.
        /// * `path` - Data folder path.
        /// * `table_name` - Data table name.
        /// * `data_name` - Data name.
        pub fn new(path: &PathBuf, table_name: String, data_name: String) -> Manager {
            let def_list = vec![Definition::new(0x0000_0000, format!("Parent Tag"), Type::String, false)];
            let children: Vec<u32> = Vec::new();
            let path_manager = PathManager::new(path.to_path_buf(), table_name, data_name.to_string());
            let data = Data::new(
                Definition {
                    tag: 0x0000_0000,
                    name: format!("Parent Tag"),
                    data_type: Type::String,
                    children: children,
                    explanation: String::new(),
                    is_base: true,
                    is_multiple: true,
                }, 
                Box::new(data_name)
            ).unwrap();
            Manager{
                parent_data: data,
                def_list: def_list,
                path_manager: path_manager,
                // is_connect: false,
            }
        }

        /// add definition
        /// * `tags` - definition tag.
        /// * `data_typw` - definition data type.
        /// * `is_multiple` - definition data is able to multiple.
        pub fn add_def(&mut self, tag: u32, name: String, data_type: Type, is_multiple: bool) -> Result<()> {
            if self.def_list.iter().find(|x| x.tag == tag).is_some() {
                panic!("tag {:0x} is already defined", tag);
            }
            self.def_list.push(Definition::new(tag, name, data_type, is_multiple));
            Ok(())
        }

        /// get definition
        /// * `tags` - definition tag.
        pub fn get_def(&self, tag: &u32) -> Result<&Definition>{
            let d = self.def_list.iter().find(|x| x.tag == *tag);
            match d {
                Some(def) => Ok(def),
                None => {panic!("tag {:0x} is not defined", tag);},
            }
        }

        /// get definition by mut
        pub fn get_def_mut(&mut self, tag: &u32) -> Option<&mut Definition>{
            self.def_list.iter_mut().find(|x| x.tag == *tag)
        }

        pub fn get_def_tag_list(&self) -> Vec<u32> {
            let mut vec = Vec::new();
            for def in &self.def_list {
                vec.push(def.tag);
            }
            vec
            // self.def_list.clone()
        }

        /// add child data.
        /// * `data` - added data.
        /// * `root` - child data root.
        pub fn add_child(&mut self, data: Data, root: &mut DataRoot) {
            self.parent_data.add_child(root, data);
        }

        /// get data.
        /// * `root` - data root.
        pub fn get_data(&self, root: &DataRoot) -> Option<&Data> {
            self.parent_data.get_child(root)
        }

        /// write definition on file.
        pub fn write_def(&self) -> Result<()> {
            let mut b = BinaryManager::new();
            for def in &self.def_list{
                // Exclude base definition.
                if def.is_base() {
                    continue;
                }

                b.create_child(Mode::Write);
                // STX
                b.get_child()?.add_start_data();
                // Tag
                b.get_child()?.add_u32(def.tag)?;

                b.get_child()?.add_str(&def.name)?;
                // Data type
                b.get_child()?.add_u8(def.get_type_num());
                // Is multiple
                b.get_child()?.add_bool(def.is_multiple);
                // Explanation
                b.get_child()?.add_str(&def.explanation)?;
                // Children tags.
                b.get_child()?.add_usize(def.children.len())?;
                for child in &def.children {
                    b.get_child()?.add_u32(*child)?;
                }
                // ETX, Check sum
                b.end_child_and_add_data()?;
            }
            b.write(&self.path_manager.get_def_path()?);
            Ok(())
        }

        /// Read definition data from file
        pub fn read_def_from_file(&mut self) -> Result<()> {
            let mut binary_manager = BinaryManager::new();
            binary_manager.read_to_end(&self.path_manager.get_def_path()?)?;
            self.read_def_from_binary(&mut binary_manager)?;
            Ok(())
        }

        /// Read definition data from binary data.
        /// * `b` - binary data manager.
        pub fn read_def_from_binary(&mut self, b: &mut BinaryManager) -> Result<()> {
            while b.next_stx_index() {
                b.create_child(Mode::Read);

                // STX
                b.get_child()?.read_u8();
                // Tag
                let tag = b.get_child()?.read_u32()?;

                let name = b.get_child()?.read_str()?;
                // Data type
                let data_type: Type = Type::u8_to_type(b.get_child()?.read_u8());
                // Is multiple
                let is_multiple = b.get_child()?.read_u8() != 0;

                // make new definition structure.
                let mut def = Definition::new(tag, name, data_type, is_multiple);

                // Explanation
                def.explanation = b.get_child()?.read_str()?;
                // Children Tags
                for _ in 0..b.get_child()?.read_u128()? as usize {
                    def.children.push(b.get_child()?.read_u32()?);
                }
                // ETX
                b.get_child()?.read_u8();
                // Check sum and get child data.
                b.read_child()?;

                self.def_list.push(def);
            }

            Ok(())
        }

        /// write data in file.
        pub fn write_data(&self) -> Result<()> {
            let mut binary_manager = BinaryManager::new();
            for child in self.parent_data.get_children() {
                self.write_child_data(child, &mut binary_manager)?
            }
            Ok(())
        }

        /// write child data in file.
        /// * `data` - written data.
        /// * `b` - binary data manager.
        pub fn write_child_data(&self, data: &Data, b: &mut BinaryManager) -> Result<()> {
            // able to write only defined tag.
            b.create_child(Mode::Write);
            // STX
            b.get_child()?.add_start_data();
            // Tag
            b.get_child()?.add_u32(data.get_def().tag)?;

            // Write value in file
            for bite in data.get_bite_value() {
                b.get_child()?.add_u8(*bite);
            }

            // Children
            for child in data.get_children() {
                self.write_child_data(child, b.get_child()?)?;
            }

            // ETX, Check sum
            b.end_child_and_add_data()?;
            b.write(&self.path_manager.get_data_path()?);
            Ok(())
        }

        /// read data from file.
        pub fn read_data(&mut self) -> Result<()> {
            let mut binary_manager = BinaryManager::new();
            binary_manager.read_to_end(&self.path_manager.get_data_path()?)?;
            self.read_child_data(&[], &mut binary_manager)?;
            Ok(())
        }

        /// read child data from file.
        /// * `index` - Index from parent to here.
        /// * `b` - Binary manager.
        pub fn read_child_data(&mut self, index: &[usize], b: &mut BinaryManager) -> Result<bool> {
            while b.next_stx_index() {
                b.create_child(Mode::Read);
                // STX
                b.get_child()?.read_u8();
                // Tag
                let tag = b.get_child()?.read_u32()?;

                let def = self.get_def(&tag)?;

                // data value
                let value = match def.get_type() {
                    Type::Int => b.get_child()?.read_u8_vec(4),
                    Type::Float => b.get_child()?.read_u8_vec(4),
                    Type::String => {
                        let len_bite = b.get_child()?.read_u128()?;
                        let len = len_bite as usize;
                        let vec: Vec<u8> = Vec::new();
                        let mut vec = u128_to_slice(len_bite, &mut vec.clone(), 120)?;
                        let vec2 = &mut b.get_child()?.read_u8_vec(len).clone();
                        vec.append(vec2);
                        vec
                    },
                };

                let data = Data::read_binary(def.clone(), value);

                // add data in parent.
                let parent = self.parent_data.get_child_mut_by_index(index)
                    .expect("Fail to read tag from file.");
                parent.add_child(&mut [], data);

                // Children
                let mut index_vec = index.to_vec();
                index_vec.push(parent.get_children().len() - 1);
                while self.read_child_data(&mut index_vec, b.get_child()?)? {
                    continue;
                }

                // ETX
                b.get_child()?.read_u8();

                // Check sum and read child data.
                b.read_child()?;
                return Ok(true);
            }
            Ok(false)
        }
    }
}
