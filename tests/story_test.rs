
extern crate our_fractal_core;

use our_fractal_core::{Manager, Data, Type};
use std::path::PathBuf; 

#[cfg(test)]
extern crate speculate;

#[cfg(test)]
use speculate::speculate;

#[cfg(test)]
speculate! {
    describe "story" {
        it "Demonstration." {
            // Demonstration.

            // 1. Start Our Fractal.
            let path: &PathBuf = &PathBuf::from("./files/mfd_test01");
            let mut manager: Manager = Manager::new(path, "data_test".to_string(), "data_name".to_string());

            // 2. Add definition.
            manager.add_def(0xabcd_abcd, format!("abcd data"), Type::Int, false).unwrap();
            manager.add_def(0x1234_5678, format!("1234 data"), Type::String, false).unwrap();
            manager.get_def_mut(&0xabcd_abcd).unwrap().children.push(0x1234_5678);

            // 3. R/W definition on binary file.
            manager.write_def().unwrap();
            manager.read_def_from_file().unwrap();

            // 4. Check definition. 
            println!("Def1[0xabcd_abcd]: {:?}", manager.get_def(&0xabcd_abcd).unwrap());
            println!("Def2[0x1234_5678]: {:?}", manager.get_def(&0x1234_5678).unwrap());

            // 5. Add child data.
            manager.add_child(
                Data::new( 
                    (*manager.get_def(&0xabcd_abcd).unwrap()).clone(), 
                    Box::new(3)
                ).unwrap(),
                &mut[]
            );
            manager.add_child(
                Data::new(
                    (*manager.get_def(&0x1234_5678).unwrap()).clone(), 
                    Box::new(format!("taro"))).unwrap(), 
                &mut[(0xabcd_abcd, None)]
            );

            // 6. R/W data on binary file.
            manager.write_data().unwrap();
            manager.read_data().unwrap();

            // 7. Check data.
            assert_eq!(manager.get_data(&[(0xabcd_abcd, None)])
                .unwrap().get_value().unwrap().downcast_ref::<i32>().unwrap(), &3);
            assert_eq!(manager.get_data(&[(0xabcd_abcd, None), (0x1234_5678, None)])
                .unwrap().get_value().unwrap().downcast_ref::<String>().unwrap(), "taro");
        }
    }
}