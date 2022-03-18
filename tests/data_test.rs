
extern crate our_fractal_core;

use our_fractal_core::{Data, Definition, Type};

#[cfg(test)]
extern crate speculate;

#[cfg(test)]
use speculate::speculate;

#[cfg(test)]
speculate! {
    describe "data" {
        it "int data r/w" {
            let def = Definition::new(0xaaaa_bbbb, format!("Int test"), Type::Int, false);
            let data = Data::new(def, Box::new(3_i32)).unwrap();
            assert_eq!(data.get_value().unwrap().downcast_ref::<i32>().unwrap(), &3_i32);
        }

        it "float data r/w" {
            let def = Definition::new(0xaaaa_bbbb, format!("Float test"), Type::Float, false);
            let data = Data::new(def, Box::new(4.5_f32)).unwrap();
            assert_eq!(data.get_value().unwrap().downcast_ref::<f32>().unwrap(), &4.5_f32);
        }

        it "string data r/w" {
            let def = Definition::new(0xaaaa_bbbb, format!("String test"), Type::String, false);
            let data = Data::new(def, Box::new(format!("taro"))).unwrap();
            assert_eq!(data.get_value().unwrap().downcast_ref::<String>().unwrap(), &format!("taro"));
        }
    }
}