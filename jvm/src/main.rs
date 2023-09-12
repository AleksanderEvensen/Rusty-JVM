mod jvm;
mod utils;

use clap::Parser;
use jvm::JVM;
use jvm_parser::{classfile::JavaClass, jar::JarFile};
use std::path::PathBuf;

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
            Some(PathBuf::from("./java/jvm-test.jar"))
        })
        .unwrap();

    let mut jvm = JVM::new();

    jvm.add_jar(JarFile::from_file(&PathBuf::from("./rt.jar")).unwrap())
        .unwrap();

    let file_ext = file.extension().unwrap();

    if file_ext == "class" {
        jvm.add_class(JavaClass::from_file(&file).unwrap()).unwrap();
    } else if file_ext == "jar" || file_ext == "zip" {
        jvm.add_jar(JarFile::from_file(&file).unwrap()).unwrap();
    }

    jvm.run().unwrap();
}
