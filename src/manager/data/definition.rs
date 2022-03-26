

pub mod child;
pub mod data_type;
pub mod multi_type;

pub mod definition {
    use crate::manager::data::definition::child::child::Child;
    use crate::manager::data::definition::data_type::data_type::Type;
    // use crate::manager::data::definition::multi_type::multi_type::MultiType;

    // use serde::{Serialize, Deserialize};

    /// Data definition
    #[derive(Debug, Clone)]
    pub struct Definition {
        /// tag value.  
        /// The first 4 digits : the group number  
        /// The last 4 digits  : the element number.
        pub tag: u32,

        pub name: String,
        /// value type
        pub data_type: Type,
        /// explanation
        pub explanation: String,

        pub is_base: bool,
        /// tag number of children
        pub children: Vec<Child>,
    }

    impl Definition {
        /// Create new definition
        /// * `tag` - definition tag.
        /// * `name` - definition name.
        /// * `data_type` - definition data type.
        pub fn new(tag: u32, name: String, data_type: Type) -> Definition {
            Definition {
                tag: tag,
                name: name,
                data_type: data_type,
                explanation: format!(""),
                is_base: false,
                children: vec![],
            }
        }

        /// definition name
        pub fn get_name(&self) -> &String {
            &self.name
        }

        /// get type
        pub fn get_type(&self) -> &Type {
            &self.data_type
        }

        /// Get group value(int)
        pub fn get_group_num(&self) -> u32{
            let r = self.tag;
            r>>16
        }

        /// Get element value(int).
        pub fn get_element_num(&self) -> u32{
            self.tag &0xffff
        }

        /// Get element value by Hexadecimal.
        pub fn get_element_value(&self) -> String{
            format!("{:04x}", (self.tag &0xffff))
        }

        /// Get group value by Hexadecimal.
        pub fn get_group_value(&self) -> String{
            let r = self.tag;
            format!("{:04x}", r>>16)
        }

        /// get type by binary.
        pub fn get_type_num(&self) -> Option<u8> {
            match self.data_type {
                Type::Int => Some(0x00),
                Type::Float => Some(0x01),
                Type::String => Some(0x02),
                Type::Err => None,
            }
        }

        /// get explanation about definition.
        pub fn get_explanation(&self) -> &String {
            &self.explanation
        }

        /// set explanation.
        /// * `explanation` - explanation of tag.
        pub fn set_explanation(&mut self, explanation: String) {
            self.explanation = explanation
        }

        pub fn is_base(&self) -> bool {
            self.is_base
        }
    }
}
