
pub mod definition {

    use std::error;

    use serde::{Serialize, Deserialize};

    type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

    #[derive(Debug, Clone)]
    pub struct Child {
        pub tag: u32,
        pub multi_type: MultiType,
    }

    impl Child {
        /// get multi type from u8 num.
        pub fn get_multi_type_u8_num(&self) -> Result<u8> {
            match self.multi_type {
                MultiType::Single => Ok(0x00),
                _ => Err("Multi type convert error")?
            }
        }
        
        /// create child.
        /// * `tag` - Child tag.
        /// * `tag` - Child multi type.
        pub fn create(tag: u32, multu_type: MultiType) -> Child {
            Child {
                tag: tag,
                multi_type: multu_type,
            }
        }

        /// create child by binary data.
        /// * `tag` - Child tag.
        /// * `tag` - Child multi type which binary.
        pub fn create_by_binary(tag: u32, multi_type_binnary: u8) -> Child {
            let multi_type = match multi_type_binnary {
                0x00 => MultiType::Single,
                _ => MultiType::Err
            };
            Child {
                tag: tag,
                multi_type: multi_type,
            }
        }
    }

    /// type in data value
    #[derive (Clone, Copy, Debug, Serialize, Deserialize)]
    pub enum Type {
        Int,
        Float,
        String,
        Err,
    }

    impl Type {
        pub fn u8_to_type(buf: u8) -> Type {
            match buf {
                0x00 => Type::Int,
                0x01 => Type::Float,
                0x02 => Type::String,
                _ => Type::Err,
            }
        }
    }

    #[derive (Clone, Copy, Debug, Serialize, Deserialize)]
    pub enum MultiType {
        Single,
        Dictionary,
        Err,
    }


    impl MultiType {
        pub fn u8_to_multi_type(buf: u8) -> MultiType {
            match buf {
                0x00 => MultiType::Single,
                _ => MultiType::Err,
            }
        }
    }

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
        /// is multiple.
        pub is_multiple: bool,

        pub is_base: bool,
        /// tag number of children
        pub children: Vec<Child>,
    }

    impl Definition {
        // Create new definition
        pub fn new(tag: u32, name: String, data_type: Type, is_multiple: bool) -> Definition {
            let vec: Vec<Child> = Vec::new();
            Definition {
                tag: tag,
                name: name,
                data_type: data_type,
                explanation: format!(""),
                is_multiple: is_multiple,
                is_base: false,
                // children: vec,
                children: vec,
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
