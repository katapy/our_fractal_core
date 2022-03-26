
pub mod multi_type {

    use serde::{Serialize, Deserialize};

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
}