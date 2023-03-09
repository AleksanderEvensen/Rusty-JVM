mod jvm;
mod utils;

use clap::Parser;
use jvm::JVM;
use jvm_parser::{
    classfile::{
        attributes::{AttributeInfo, AttributeInfoData},
        JavaClass,
    },
    jar::JarFile,
};
use std::{
    fs::File,
    io::{BufReader, Read},
    path::PathBuf,
};

use crate::jvm::opcodes::parse_opcodes;

#[macro_export]
#[cfg(not(feature = "debug"))]
macro_rules! dbgprint {
    () => {};
    ($($arg:tt)*) => {};
}

#[derive(Parser, Debug)]
#[command(author,version,about,long_about = None)]
struct Args {
    /// The path to the .jar or .class file to be executed
    #[arg(short, long)]
    path: Option<PathBuf>,

    /// Toggles the debug prints
    #[arg(short, long)]
    debug: bool,
}

fn main() {
    let args = Args::parse();

    let file = args
        .path
        .or_else(|| {
            // Some(PathBuf::from(
            //     "C:/Users/ahse0/AppData/Roaming/.minecraft/versions/1.19.3./1.19.3.jar",
            // ))
            Some(PathBuf::from("./rt.jar"))
        })
        .unwrap();

    let mut jvm = JVM::new();

    let file_ext = file.extension().unwrap();

    if file_ext == "class" {
        jvm.add_class(JavaClass::from_file(&file).unwrap()).unwrap();
    } else if file_ext == "jar" || file_ext == "zip" {
        jvm.add_jar(JarFile::from_file(&file).unwrap()).unwrap();
    }

    jvm.get_classes()
        .iter()
        .for_each(|(class_name, java_class)| {
            println!("Verifying: {class_name}");
            java_class.methods.iter().for_each(|method| {
                if let Some(code_attrib) = method.attributes.iter().find(|v| match v.attribute {
                    AttributeInfoData::Code(_) => true,
                    _ => false,
                }) {
                    let AttributeInfoData::Code(code) = &code_attrib.attribute else {
						unreachable!("This should never be reachable");
					};

                    parse_opcodes(&code.code).unwrap();
                }
            })
        });

    // jvm.run().unwrap();
}
