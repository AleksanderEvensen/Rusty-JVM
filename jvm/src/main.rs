mod jvm;
mod utils;

use clap::Parser;
use jvm_parser::jar::JarFile;
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
    path: Option<PathBuf>,

    /// Toggles the debug prints
    #[arg(short, long)]
    debug: bool,
}

fn main() {
    let jar_file = JarFile::from_file(&PathBuf::from("./java/jvm-test.jar")).unwrap();
    println!("Jar File: \n{:#?}", jar_file);
}
