
pub mod definition {
    /// type in data value
    #[derive (Clone, Copy, Debug)]
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
    #[derive(Clone, Debug)]
    pub struct Definition {
        /// tag value.  
        /// The first 4 digits : the group number  
        /// The last 4 digits  : the element number.
        pub tag: u32,
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
        pub fn new(tag: u32, data_type: Type, is_multiple: bool) -> Definition {
            let vec: Vec<u32> = Vec::new();
            Definition {
                tag: tag,
                data_type: data_type,
                explanation: format!(""),
                is_multiple: is_multiple,
                is_base: false,
                children: vec,
            }
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

        /*
        pub fn create() {

        }

        pub fn alter(tag: u32, ) {

        }
        
        pub fn drop() {

        }
        */
    }
}
