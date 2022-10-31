use crate::jvm::{DebugLevel, JVM};
use clap::Parser;
use jvm_parser::{self, ClassFile};
use std::path::PathBuf;

pub mod java_mappings;
mod jvm;
pub mod utils;

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

    let mut jvm = JVM::new(class_file);

    jvm.set_debug_level(if args.debug {
        DebugLevel::Debug
    } else {
        DebugLevel::None
    });
    jvm.run_main().unwrap();
}
