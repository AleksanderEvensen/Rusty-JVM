pub mod java;
pub mod javax;

extern crate macros;
use macros::generate_mappings;

use crate::jvm::traits::JavaClass;

pub fn get_class_constructor(path: &str) {
    let constructor: fn() -> Box<dyn JavaClass> =
        generate_mappings!(path, create_class, "jvm/src/java_mappings");
    println!("{:?}", constructor);
}
