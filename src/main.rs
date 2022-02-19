
pub mod manager;

// use manager::manager::{Manager, static_manager};
use manager::manager::{Manager};
use manager::data::data::Data;
use manager::data::definition::definition::{Definition, Type};

#[tokio::main]
async fn main() {
    /*
    let a = async{
        static_manager().await.add_def(0xabcd_abcd, format!("Int test"), Type::Int, false).unwrap();
        static_manager().await.get_def_mut(&0xabcd_abcd).unwrap().set_explanation(format!("test"));
        static_manager().await.add_def(0x1234_5678, format!("child test"), Type::Int, false).unwrap();
        println!("{}", static_manager().await.get_def(&0xabcd_abcd).unwrap().explanation);

        static_manager().await.write_def().expect("Write def file error");
        static_manager().await.read_def_from_file().expect("Read def file error");

        let path = &std::path::PathBuf::from("./files/mfd_test01");
        let mut manager = Manager::new(path, format!("table_test"), format!("data_test"));
        manager.read_def_from_file().expect("Read def file error");
        println!("{}", manager.get_def(&0xabcd_abcd).unwrap().explanation);

        static_manager().await.add_child(
            Data::new(Definition::new(0xabcd_abcd, format!("children test"), Type::Int, false), Box::new(3)).unwrap(), 
            &mut[]
        );
        println!("{:?}", static_manager().await.get_data(&[(0xabcd_abcd, None)]).unwrap().get_value().unwrap().downcast_ref::<i32>().unwrap());

        static_manager().await.add_child(
            Data::new(Definition::new(0x1234_5678, format!("string test"), Type::String, false), Box::new(format!("taro"))).unwrap(), 
            &mut[(0xabcd_abcd, None)]
        );
        println!("{:?}", static_manager().await.get_data(&[(0xabcd_abcd, None), (0x1234_5678, None)])
            .unwrap().get_value().unwrap().downcast_ref::<String>().unwrap());

        static_manager().await.write_data().unwrap();
        static_manager().await.read_data().unwrap();
    };

    a.await
    */
}