
pub mod manager;

use manager::manager::Manager;
use manager::data::data::Data;
use manager::data::definition::definition::{Definition, Type};

use once_cell::sync::Lazy;
use std::{sync::Mutex, collections::HashMap};
use std::any::{Any, TypeId};

static GLOBAL_DATA: Lazy<Mutex<Manager>> = Lazy::new(|| {
    let path = &std::path::PathBuf::from("./files/mfd_test01");
    let manager = Manager::new(path, format!("table_test"), format!("data_test"));
    Mutex::new(manager)
});

fn main() {
    GLOBAL_DATA.lock().unwrap().add_def(0xabcd_abcd, Type::Int, false).unwrap();
    GLOBAL_DATA.lock().unwrap().get_def_mut(&0xabcd_abcd).unwrap().set_explanation(format!("test"));
    GLOBAL_DATA.lock().unwrap().add_def(0x1234_5678, Type::Int, false).unwrap();
    println!("{}", GLOBAL_DATA.lock().unwrap().get_def(&0xabcd_abcd).unwrap().explanation);

    GLOBAL_DATA.lock().unwrap().write_def().expect("Write def file error");
    GLOBAL_DATA.lock().unwrap().read_def_from_file().expect("Read def file error");

    let path = &std::path::PathBuf::from("./files/mfd_test01");
    let mut manager = Manager::new(path, format!("table_test"), format!("data_test"));
    manager.read_def_from_file().expect("Read def file error");
    println!("{}", manager.get_def(&0xabcd_abcd).unwrap().explanation);

    GLOBAL_DATA.lock().unwrap().add_child(
        Data::new(Definition::new(0xabcd_abcd, Type::Int, false), Box::new(3)).unwrap(), 
        &mut[]
    );
    println!("{:?}", GLOBAL_DATA.lock().unwrap().get_data(&[(0xabcd_abcd, None)]).unwrap().get_value().unwrap().downcast_ref::<i32>().unwrap());

    GLOBAL_DATA.lock().unwrap().add_child(
        Data::new(Definition::new(0x1234_5678, Type::String, false), Box::new(format!("taro"))).unwrap(), 
        &mut[(0xabcd_abcd, None)]
    );
    println!("{:?}", GLOBAL_DATA.lock().unwrap().get_data(&[(0xabcd_abcd, None), (0x1234_5678, None)])
        .unwrap().get_value().unwrap().downcast_ref::<String>().unwrap());

    GLOBAL_DATA.lock().unwrap().write_data().unwrap();
    GLOBAL_DATA.lock().unwrap().read_data().unwrap();
}
