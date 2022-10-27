// use crate::jvm::traits::{JavaClass, JavaClassInitContext};

// #[macro_export]
// macro_rules! generate_mappings {
//     ($namespace:expr, $($class:ident),*) => {
// 		vec![
// 			$((format!("{}{}",$namespace,stringify!($class)), Box::new($class))),*
// 		]
//     };
// }

pub mod java;

// pub fn get_method_map() {
//     let methods: Vec<(String, fn(JavaClassInitContext) -> Box<dyn JavaClass>)> = vec![];

//     methods.append(&mut self::java::io::get_method_map());
//     methods.append(&mut self::java::lang::get_method_map());

//     return methods;
// }
