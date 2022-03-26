

pub mod child{

    use std::error;

    use crate::manager::data::definition::multi_type::multi_type::MultiType;
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
}