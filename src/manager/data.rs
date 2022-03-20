
pub mod definition;

pub mod data {
    use crate::manager::data::definition::definition::{Definition, Type};

    use std::error;
    use std::any::{Any};

    // Change the alias to `Box<dyn error::Error>`.
    type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

    #[derive (Clone, Debug)]
    pub struct DataRoot {
        pub tag: u32,
        pub id_data: Option<Data>,
    }

    #[derive (Clone, Debug)]
    pub struct Data {
        def: Definition,
        value: Vec<u8>,
        children: Vec<Data>,
    }

    impl Data {
        /// Create new data.
        /// * `def` - definition.
        /// * `boxed_value` - data value.
        pub fn new(def: Definition, boxed_value: Box<dyn Any>) -> Result<Data> {
            let children: Vec<Data> = Vec::new();
            match def.get_type() {
                Type::Int => match &(*boxed_value).downcast_ref::<i32>() {
                    Some(value) => Ok({
                        Data{
                            def: def,
                            value: value.to_le_bytes().to_vec(),
                            children: children,
                    }}),
                    None => return Err(format!("Cannot convert to int"))?,
                },
                Type::Float => match &(*boxed_value).downcast_ref::<f32>() {
                    Some(value) => Ok({
                        Data{
                            def: def,
                            value:  value.to_le_bytes().to_vec(),
                            children:children,
                    }}),
                    None => return Err(format!("Cannot convert to float"))?,
                },
                Type::String => match &(*boxed_value).downcast_ref::<String>() {
                    Some(value) => Ok({
                        Data{
                            def: def,
                            value: value.as_bytes().to_vec(),
                            children: children,
                    }}),
                    None => return Err(format!("Cannot convert to string"))?,
                },
                Type::Err => return Err(format!("Definition is incorrect"))?
            }
        }

        /// Create new data. It use when read binary data
        /// * `def` - Data definition
        /// * `value` - data value which type is binary.
        pub fn read_binary(def: Definition, value: Vec<u8>) -> Data {
            let children: Vec<Data> = Vec::new();
            Data {
                def: def,
                value: value,
                children: children,
            }
        }

        /// get data definition
        pub fn get_def(&self) -> &Definition {
            &self.def
        }

        /// get data value
        pub fn get_value(&self) -> Result<Box<dyn Any>> {
            match self.get_def().get_type() {
                Type::Int => Ok(Box::new(i32::from_le_bytes(self.value[0..4].try_into()?))),
                Type::Float => Ok(Box::new(f32::from_le_bytes(self.value[0..4].try_into()?))),
                Type::String => Ok(Box::new(String::from_utf8(self.value.clone())?)),
                Type::Err => Err("Data type is incorrect.")?
            }
        }

        /// get child tag data
        /// * `tags` - tag and child data pairs.
        pub fn get_child(&self, root: Vec<&DataRoot>) -> Option<&Data>{
            if root.is_empty() {
                Some(&self)
            }
            else {
                match &root[0].id_data {
                    // Selects the values ​​requested by the tag that have the same tuple data.
                    Some(id_data) => self.children.iter().find(|e| 
                        e.def.tag==root[0].tag && e.is_equal_child(id_data))?.get_child(root[1..].to_vec()),
                    // Select the values ​​requested by the tag if tuple data is None.
                    None => self.children.iter().find(|e| e.def.tag==root[0].tag)?.get_child(root[1..].to_vec())
                }
            }
        }

        /// get child tag data mut
        /// * `root` - data root.
        pub fn get_child_mut(&mut self, root: Vec<&DataRoot>) -> Option<&mut Data>{
            if root.is_empty() {
                Some(self)
            }
            else {
                match &root[0].id_data {
                    // Selects the values ​​requested by the tag that have the same tuple data.
                    Some(data) => self.children.iter_mut().find(|e| 
                        e.def.tag==root[0].tag && e.is_equal_child(data))?.get_child_mut(root[1..].to_vec()),
                    // Select the values ​​requested by the tag if tuple data is None.
                    None => self.children.iter_mut().find(|e| e.def.tag==root[0].tag)?.get_child_mut(root[1..].to_vec())
                }
            }
        }

        /// get child by index.
        /// * `index` - data index.
        pub fn get_child_mut_by_index(&mut self, index: &[usize]) -> Option<&mut Data> {
            if index.is_empty() {
                Some(self)
            }
            else {
                self.children[index[0]].get_child_mut_by_index(&index[1..])
            }
        }


        /// Return true if this child and data value is same.
        /// * `data` - Compared data.
        fn is_equal_child(&self, data: &Data) -> bool {
            match self.children.iter().find(|e| e.def.tag==data.def.tag) {
                Some(child) => data.value == child.value,
                None => false,
            }
        }

        /// Add child data in this.
        /// * `data` - Input data for child.
        pub fn add_child(&mut self, root: Vec<&DataRoot>, data: &mut Data, id_data: Option<Data>) {
            match id_data {
                Some(id_data) => data.children.push(id_data.clone()),
                None => (),
            }
            self.get_child_mut(root).unwrap().children.push(data.clone());
        }

        /// get children
        pub fn get_children(&self) -> &Vec<Data> {
            &self.children
        }

        /// get value (type is binary)
        pub fn get_bite_value(&self) -> &Vec<u8> {
            &self.value
        }
    }
}