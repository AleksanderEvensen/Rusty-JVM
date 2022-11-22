// use crate::jvm::JVM;
use clap::Parser;
use jvm_parser::jar::JarFile;
// use jvm_parser::{self, ClassFile};
use std::path::PathBuf;

// mod jvm;
// pub mod utils;

#[macro_export]
#[cfg(feature = "debug")]
macro_rules! dbgprint {
    () => {
        println!()
    };

	($($arg:tt)*) => {
		println!($($arg)*)
	}
}

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
    // let args = Args::parse();

    // let file_path = args
    //     .path
    //     .or_else(|| Some("./java/MyProgram.class".into()))
    //     .unwrap();

    // let class_file = ClassFile::from_file(&file_path).unwrap();

    let jar_file = JarFile::from_file(&PathBuf::from("./java/jvm-test.jar")).unwrap();

    // dbgprint!(
    //     "Magic: {:X?}\nVersion: {} : {}",
    //     &class_file.magic,
    //     &class_file.major_version,
    //     &class_file.minor_version
    // );
    // #[cfg(feature = "debug")]
    // {
    //     class_file
    //         .constant_pool
    //         .pool_entries
    //         .iter()
    //         .enumerate()
    //         .for_each(|(i, cp_info)| dbgprint!("[{}] cp_info = {:?}", i + 1, cp_info));
    // }

    // let mut jvm = JVM::new();
    // jvm.add_class_file(class_file).add_class_file(class_file_2);
    // jvm.run();

    // println!("{:#?}", class_file);
}
