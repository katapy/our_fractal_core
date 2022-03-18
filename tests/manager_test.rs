
// Test Command
// cargo test -- --test-threads=1 > test.txt

extern crate our_fractal_core;

use our_fractal_core::{Manager, Type};

#[cfg(test)]
extern crate speculate;

#[cfg(test)]
use speculate::speculate;

#[cfg(test)]
speculate! {
    describe "definition" {
        it "tag list" {
            let path = "./test";
            let table_name = format!("test");
            let data_name = format!("test");
            let path = &std::path::PathBuf::from(path);
            let mut manager = Manager::new(path, table_name, data_name);
            manager.add_def(0xabcd_abcd, format!("Int test"), Type::Int, false).unwrap();
            manager.add_def(0x1234_5678, format!("child test"), Type::Int, false).unwrap();
            assert_eq!(manager.get_def_tag_list(), vec![0x0000_0000, 0xabcd_abcd, 0x1234_5678]);
        }
        it "add definition child" {
            let path = "./test";
            let table_name = format!("test");
            let data_name = format!("test");
            let path = &std::path::PathBuf::from(path);
            let mut manager = Manager::new(path, table_name, data_name);
            manager.add_def(0xabcd_abcd, format!("Int test"), Type::Int, false).unwrap();
            manager.add_def(0x1234_5678, format!("child test"), Type::Int, false).unwrap();
            manager.add_def_child(&0xabcd_abcd, 0x1234_5678);
            assert_eq!(manager.get_def(&0xabcd_abcd).unwrap().children, vec![0x1234_5678]);
        }
    }
}