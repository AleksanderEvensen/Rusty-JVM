pub mod java;
pub mod javax;

extern crate macros;
use macros::generate_mappings;

use crate::jvm::traits::JavaClass;

pub fn get_class_constructor(path: &str) {
    let test: fn() -> Box<dyn JavaClass> =
        generate_mappings!(path, create_class, "jvm/src/java_mappings");
    println!("{:?}", test);
}
