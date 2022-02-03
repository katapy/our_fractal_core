
pub mod definition {

    use serde::{Serialize, Deserialize};

    /// type in data value
    #[derive (Clone, Copy, Debug, Serialize, Deserialize)]
    pub enum Type {
        Int,
        Float,
        String,
    }

    impl Type {
        pub fn u8_to_type(buf: u8) -> Type {
            match buf {
                0x00 => Type::Int,
                0x01 => Type::Float,
                0x02 => Type::String,
                _ => panic!("convert error"),
            }
        }
    }

    /// Data definition
    #[derive(Clone, Debug, Serialize, Deserialize)]
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
        pub children: Vec<u32>,
    }

    impl Definition {
        // Create new definition
        pub fn new(tag: u32, name: String, data_type: Type, is_multiple: bool) -> Definition {
            let vec: Vec<u32> = Vec::new();
            Definition {
                tag: tag,
                name: name,
                data_type: data_type,
                explanation: format!(""),
                is_multiple: is_multiple,
                is_base: false,
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
        pub fn get_type_num(&self) -> u8 {
            match self.data_type {
                Type::Int => 0x00,
                Type::Float => 0x01,
                Type::String => 0x02,
            }
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

    #[cfg(test)]
    extern crate speculate;

    #[cfg(test)]
    use speculate::speculate;

    // Test Command
    // cargo test -- --test-threads=1 > test.txt
    #[cfg(test)]
    speculate! {
        describe "definition" {
            it "def name" {
                let def = Definition::new(0xaaaa_bbbb, format!("Int test"), Type::Int, false);
                assert_eq!(def.get_name(), &format!("Int test"));
            }

            it "def is base" {
                let def = Definition::new(0xaaaa_bbbb, format!("Int test"), Type::Int, false);
                assert_eq!(def.is_base(), false);
            }
        }
    }
}
