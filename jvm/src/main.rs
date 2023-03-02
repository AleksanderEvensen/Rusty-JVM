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
    let big_bytes = std::fs::read(PathBuf::from("./java/out/com/ahse/jvm/Main.class")).unwrap();
    // let big_bytes = std::fs::read(PathBuf::from("./ORBUtilSystemException.class")).unwrap();

    let iters = 100;

    println!("Benching the nom implementation");
    let start = std::time::Instant::now();
    for _ in 0..iters {
        class_parser::parse(&big_bytes[..]).unwrap();
    }
    let delta = std::time::Instant::now() - start;
    println!(
        "Results for 'nom': {}s on {iters} iterations",
        delta.as_secs_f64()
    );

    println!("Benching the \"old\" implementation");
    let start = std::time::Instant::now();
    for _ in 0..iters {
        JavaClass::from_bytes(&big_bytes).unwrap();
    }
    let delta = std::time::Instant::now() - start;
    println!(
        "Results for 'old': {}s on {iters} iterations",
        delta.as_secs_f64()
    );

    // let (_, data) = class_parser::parse(&bytes[..]).unwrap();

    // println!("{data:#?}");

    // let args = Args::parse();

    // let file = args
    //     .path
    //     .or_else(|| Some(PathBuf::from("./java/jvm-test.jar")))
    //     .unwrap();

    // let mut jvm = JVM::new();

    // let file_ext = file.extension().unwrap();

    // if file_ext == "class" {
    //     jvm.add_class(JavaClass::from_file(&file).unwrap()).unwrap();
    // } else if file_ext == "jar" || file_ext == "zip" {
    //     jvm.add_jar(JarFile::from_file(&file).unwrap()).unwrap();
    // }

    // jvm.run().unwrap();
}
