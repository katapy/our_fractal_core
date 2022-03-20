
extern crate our_fractal_core;

use our_fractal_core::{Manager, Data, Type, Child, MultiType, DataRoot};

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
            manager.add_def_child(&0xabcd_abcd, Child::create(0x1234_5678, MultiType::Single));
            assert_eq!(manager.get_def(&0xabcd_abcd).unwrap().children[0].tag, 0x1234_5678);
        }

        it "dic data test"  {
            let path = "./test";
            let table_name = format!("test");
            let data_name = format!("test");
            let path = &std::path::PathBuf::from(path);
            let mut manager = Manager::new(path, table_name, data_name);
            manager.add_def(0xabcd_abcd, format!("Int test"), Type::Int, false).unwrap();
            manager.add_def(0x1234_5678, format!("child test"), Type::Int, false).unwrap();

            manager.add_def(0x0000_0020, format!("Dictinary ID"), Type::String, false).unwrap();

            manager.add_def_child(&0xabcd_abcd, Child::create(0x1234_5678, MultiType::Dictionary));
            
            let id_data_001 = Data::new(manager.get_def(&0x0000_0020).unwrap().clone(), Box::new(format!("id_data_001"))).unwrap();
            let id_data_002 = Data::new(manager.get_def(&0x0000_0020).unwrap().clone(), Box::new(format!("id_data_002"))).unwrap();
            
            let root = DataRoot {
                tag: 0xabcd_abcd,
                id_data: None,
            };
            manager.add_child(
                &mut Data::new((*manager.get_def(&0xabcd_abcd).unwrap()).clone(), Box::new(-1)).unwrap(),
                vec![],
                None,
            );
            
            manager.add_child(
                &mut Data::new((*manager.get_def(&0x1234_5678).unwrap()).clone(), Box::new(1)).unwrap(),
                vec![&root.clone()],
                Some(id_data_001.clone()),
            );

            manager.add_child(
                &mut Data::new((*manager.get_def(&0x1234_5678).unwrap()).clone(), Box::new(2)).unwrap(),
                vec![&root.clone()],
                Some(id_data_002.clone()),
            );

            let child1 = DataRoot {
                tag: 0xabcd_abcd,
                id_data: None
            };
            let child2 = DataRoot {
                tag: 0x1234_5678,
                id_data: Some(id_data_001.clone()),
            };
            let child3 = DataRoot {
                tag: 0x1234_5678,
                id_data: Some(id_data_002.clone()),
            };

            let data = manager.get_data(vec![&child1.clone(), &child2.clone()]);
            assert_eq!(data.unwrap().get_value().unwrap().downcast_ref::<i32>().unwrap(), &1);
            let data2 = manager.get_data(vec![&child1.clone(), &child3.clone()]);
            assert_eq!(data2.unwrap().get_value().unwrap().downcast_ref::<i32>().unwrap(), &2);
        }
    }
}