pub mod java;

extern crate macros;
use macros::generate_mappings;

use crate::jvm::traits::{JavaClass, JavaClassInit, JavaClassInitContext};

pub fn get_class_constructor(path: &str) -> fn(JavaClassInitContext) -> Box<dyn JavaClass> {
    let constructor: fn(JavaClassInitContext) -> Box<dyn JavaClass> =
        generate_mappings!(path, construct, "jvm/src/java_mappings");
    return constructor;
}
