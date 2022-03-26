
pub mod data_type {

    use serde::{Serialize, Deserialize};
    
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
}
