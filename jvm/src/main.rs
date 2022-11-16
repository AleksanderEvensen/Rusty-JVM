use crate::jvm::JVM;
use clap::Parser;
use jvm_parser::{self, ClassFile};
use std::path::PathBuf;

mod class;
pub mod java_mappings;
mod jvm;
pub mod utils;

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
    let args = Args::parse();

    let file_path = args
        .path
        .or_else(|| Some("./java/MyProgram.class".into()))
        .unwrap();

    let class_file = ClassFile::from_file(file_path).unwrap();

    dbgprint!(
        "Magic: {:X?}\nVersion: {} : {}",
        &class_file.magic,
        &class_file.major_version,
        &class_file.minor_version
    );
    #[cfg(feature = "debug")]
    {
        class_file
            .constant_pool
            .pool_entries
            .iter()
            .enumerate()
            .for_each(|(i, cp_info)| dbgprint!("[{}] cp_info = {:?}", i + 1, cp_info));
    }

    let mut jvm = JVM::new();

    jvm.add_class_file(class_file);

    jvm.run().unwrap();

    // let (method, code) = jvm.get_main().unwrap();
    // jvm.execute_code(method, code);
}
