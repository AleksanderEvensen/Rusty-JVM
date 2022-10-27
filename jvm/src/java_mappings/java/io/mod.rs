use crate::jvm::traits::{JavaClass, JavaClassInitContext};

#[allow(non_snake_case)]
pub mod PrintStream;

// pub fn get_method_map() {
//     let methods: Vec<(String, fn(JavaClassInitContext) -> Box<dyn JavaClass>)> = vec![];

//     methods.push((
//         "java/lang/PrintStream".to_string(),
//         PrintStream::PrintStream::create_class,
//     ));

//     return methods;
// }
