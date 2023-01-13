// mod jvm;
mod utils;

use clap::Parser;
use jvm_parser::{classfile::ClassFile, jar::JarFile};
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
        .or_else(|| Some(PathBuf::from("./aic.class")))
        .unwrap();

    println!("File: {:?}", file);
    // println!("Ext: {:?}",);

    let file_ext = file.extension().unwrap();

    if file_ext == "class" {
        let _ = ClassFile::from_file(&file).unwrap();
    } else if file_ext == "jar" || file_ext == "zip" {
        let _ = JarFile::from_file(&file).unwrap();
    }
}
