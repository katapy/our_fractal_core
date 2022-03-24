
extern crate our_fractal_core;
use our_fractal_core::{Definition, Type};

#[cfg(test)]
extern crate speculate;

#[cfg(test)]
use speculate::speculate;

#[cfg(test)]
speculate! {
    describe "definition" {
        it "def name" {
            let def = Definition::new(0xaaaa_bbbb, format!("Int test"), Type::Int);
            assert_eq!(def.get_name(), &format!("Int test"));
        }

        it "def is base" {
            let def = Definition::new(0xaaaa_bbbb, format!("Int test"), Type::Int);
            assert_eq!(def.is_base(), false);
        }

        it "explanation" {
            let mut def = Definition::new(0xaaaa_bbbb, format!("Int test"), Type::Int);
            def.set_explanation(format!("exp test"));
            assert_eq!(def.get_explanation(), &format!("exp test"));
        }
    }
}