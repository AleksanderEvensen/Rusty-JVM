use core::panic;
use std::path::PathBuf;

use clap::Parser;
use jvm_parser::{
    self,
    attributes::CodeAttribute,
    content_pool::CpInfo,
    utils::{read_u1, read_u2},
    ClassFile,
};

use crate::{
    jvm::{opcodes::OpCodes, traits::JavaClass, DebugLevel, JVM},
    utils::parse_descriptor,
};

pub mod java_mappings;
mod jvm;
pub mod utils;

#[allow(dead_code)]
#[derive(Debug, Clone)]
struct StackObjectRef {
    class_name: String,
    member_name: String,
    descriptor: String,
}
/*
fn execute_code(class_file: &ClassFile, code_data: CodeAttribute) {
    let mut bytes = code_data.code;

    let mut java_objects: Vec<Box<dyn JavaClass>> = vec![];

    let mut stack: Vec<StackValue> = vec![];
    let mut frame: Vec<StackValue> = vec![StackValue::default(); code_data.max_locals as usize];

    // TODO: Implement some debug logging if the app runs with verbose enabled

    while bytes.len() > 0 {
        let opcode_byte = read_u1(&mut bytes);

        let opcode = OpCodes::from(opcode_byte);

        println!("Running OpCode: {:?}", opcode);

        match opcode {
            OpCodes::getstatic => {
                let index = read_u2(&mut bytes);

                let (_, class, name_type) = class_file
                    .constant_pool
                    .get_refs_ext_at(index)
                    .expect("Failed to fetch data from constant pool");

                let class_name = class_file
                    .constant_pool
                    .get_utf8_at(class.name_index)
                    .expect("Class Name not in const pool")
                    .data
                    .clone();
                let member_name = class_file
                    .constant_pool
                    .get_utf8_at(name_type.name_index)
                    .expect("Member Name not in const pool")
                    .data
                    .clone();

                let descriptor = class_file
                    .constant_pool
                    .get_utf8_at(name_type.descriptor_index)
                    .expect("Descriptor not in const pool")
                    .data
                    .clone();

                stack.push(StackValue::ObjectRef(StackObjectRef {
                    class_name,
                    member_name,
                    descriptor,
                }));
            }

            OpCodes::ldc => {
                let index = read_u1(&mut bytes);

                match &class_file.constant_pool.0[index as usize - 1] {
                    CpInfo::String(str) => {
                        match &class_file.constant_pool.0[str.string_index as usize - 1] {
                            CpInfo::Utf8(utf8) => stack.push(StackValue::String(utf8.data.clone())),

                            _ => {}
                        }
                    }

                    CpInfo::Float(float) => {
                        stack.push(StackValue::Float(float.bytes));
                    }

                    CpInfo::Integer(int) => {
                        stack.push(StackValue::SignedInteger(int.bytes));
                    }

                    pool_type => {
                        panic!("[ldc] Unimplemented pool type: {:?}", pool_type);
                    }
                }
            }

            OpCodes::invokevirtual => {
                let index = read_u2(&mut bytes);

                let refs = class_file.constant_pool.get_refs_at(index).unwrap();
                let class = class_file
                    .constant_pool
                    .get_class_at(refs.class_index)
                    .unwrap();
                let class_name = class_file
                    .constant_pool
                    .get_utf8_at(class.name_index)
                    .unwrap();
                let name_type = class_file
                    .constant_pool
                    .get_name_type_at(refs.name_and_type_index)
                    .unwrap();

                let name_type_name = class_file
                    .constant_pool
                    .get_utf8_at(name_type.name_index)
                    .unwrap();
                let descriptor = class_file
                    .constant_pool
                    .get_utf8_at(name_type.descriptor_index)
                    .unwrap();

                println!("{:#?}", (&class_name.data, &name_type_name.data));
                println!("Stack: {:#?}", stack);

                let descriptor_data = parse_descriptor(&descriptor.data);

                let mut args: Vec<StackValue> = vec![];

                for _ in descriptor_data.parameters {
                    args.push(stack.pop().unwrap());
                }

                let objref = if let StackValue::JavaObjectRef(value) = stack.pop().unwrap() {
                    value
                } else {
                    panic!("This isn'y supposed to happen");
                };

                println!("{:#?}", (args, objref));

                println!("");
                println!("");
                println!("");

                // let objectref_index =
                //     stack.len() as i32 - descriptor_data.parameters.len() as i32 + 1;

                // let objectref = if objectref_index >= 0 && objectref_index < stack.len() as i32 {
                //     stack.remove(objectref_index as usize)
                // } else {
                //     None
                // };

                // let objectref = if let StackValue::ObjectRef(value) = objectref.unwrap() {
                //     value
                // } else {
                //     panic!("object ref not in the stack position in invokevirtual")
                // };

                // println!("{:#?}", objectref);

                match class_name.data.as_str() {
                    "java/io/PrintStream" => match name_type_name.data.as_str() {
                        "println" => match stack.pop().unwrap() {
                            StackValue::Float(v) => {
                                println!("Float: {}", v)
                            }
                            StackValue::Integer(v) => {
                                println!("Int: {}", v)
                            }
                            StackValue::String(v) => {
                                println!("String: {}", v)
                            }
                            StackValue::Byte(v) => {
                                println!("Byte: {}", v)
                            }
                            StackValue::SignedInteger(v) => {
                                println!("Signed: {}", v)
                            }
                            invalid_data => {
                                panic!("Invalid data on the stack, {:#?}", invalid_data);
                            }
                        },
                        "print" => match stack.pop().unwrap() {
                            StackValue::Float(v) => {
                                print!("{}", v)
                            }
                            StackValue::Integer(v) => {
                                print!("{}", v)
                            }
                            StackValue::String(v) => {
                                print!("{}", v)
                            }
                            invalid_data => {
                                panic!("Invalid data on the stack, {:#?}", invalid_data);
                            }
                        },
                        v => {
                            panic!("Invokevirtual method ( {} ), isn't implemented", v);
                        }
                    },

                    "java/lang/StringBuilder" => match name_type_name.data.as_str() {
                        "append" => {
                            // if let Some(first_arg) = descriptor_data.parameters.get(0) {
                            // match first_arg {
                            //     DescriptorTypes::Class(v) => {}

                            //     DescriptorTypes::Int => {}
                            // }
                            // }

                            // descriptor_data.parameters.len();

                            println!("{:?}  {:?}", "append", descriptor.data);
                        }

                        "toString" => {
                            todo!("Implement toString for StringBuilder");
                        }
                        v => {
                            panic!("Invokevirtual method ( {} ), isn't implemented", v);
                        }
                    },

                    v => {
                        panic!("[Invokevirtual] The class ( {} ) is not implemented", v);
                    }
                }

                if class_name.data == "java/io/PrintStream" {}
            }

            OpCodes::invokespecial => {
                let index = read_u2(&mut bytes);

                let (_, class, name_type) =
                    class_file.constant_pool.get_refs_ext_at(index).unwrap();

                let class_name = class_file
                    .constant_pool
                    .get_utf8_at(class.name_index)
                    .unwrap();
                let name_type_name = class_file
                    .constant_pool
                    .get_utf8_at(name_type.name_index)
                    .unwrap();
                let descriptor = class_file
                    .constant_pool
                    .get_utf8_at(name_type.descriptor_index)
                    .unwrap();

                let objref = stack.pop();

                println!("{:?}", (descriptor, objref));

                // println!("{:#?}", (class_name, name_type_name, descriptor, objref));

                match name_type_name.data.as_str() {
                    "<init>" => match class_name.data.as_str() {
                        "java/lang/StringBuilder" => {
                            java_objects.push(Box::new(
                                crate::java_mappings::java::lang::StringBuilder::StringBuilder::new(
                                ),
                            ));
                            stack.push(StackValue::JavaObjectRef(java_objects.len() - 1));
                        }

                        v => {
                            panic!("Init Method for ( {} ) is not implemented in OpCode::invokespecial", v)
                        }
                    },

                    _ => {}
                };

                // panic!("Invokespecial not implemented");
            }

            OpCodes::bipush => {
                let byte = read_u1(&mut bytes);
                stack.push(StackValue::SignedInteger(i32::from_be_bytes([
                    0, 0, 0, byte,
                ])));
            }

            OpCodes::sipush => {
                // TODO: Change this to a short stack value(i16) instead of a signed 32 bit integer
                let value = read_u2(&mut bytes).to_be_bytes();
                let sign_extended = i32::from(i16::from_be_bytes(value));
                stack.push(StackValue::SignedInteger(sign_extended));
            }

            OpCodes::new => {
                // Should be index to class or interface
                let index = read_u2(&mut bytes) as usize;

                if let CpInfo::Class(class) = &class_file.constant_pool.0[index - 1] {
                    println!("Class: {:?}", class);
                    let class_name = class_file
                        .constant_pool
                        .get_utf8_at(class.name_index)
                        .unwrap();
                    println!("Initialize class {}", class_name.data);

                    match class_name.data.as_str() {
                        "java/io/PrintStream" => {
                            java_objects.push(Box::new(
                                crate::java_mappings::java::io::PrintStream::PrintStream::new(),
                            ));

                            stack.push(StackValue::JavaObjectRef(java_objects.len() - 1))
                        }

                        _ => {}
                    }

                    // stack.push(StackValue::ObjectRef(StackObjectRef {
                    //     class_name: class_name.data.clone(),
                    //     member_name: "".to_string(), // TODO: I dont know if this is right, but check later
                    //     descriptor: "".to_string(),
                    // }))
                } else if false { // TODO: Implement for CpInfo::Interface, when that is created
                } else {
                    panic!("OpCode::newm should have an index to either CpInfo::Class or CpInfo::Interface, not {:?}", &class_file.constant_pool.0[index - 1]);
                }
            }

            OpCodes::dup => {
                let last_element = stack.last().unwrap().clone();
                stack.push(last_element);
            }

            OpCodes::istore_(n) => {
                if let Some(int) = stack.pop() {
                    if let StackValue::SignedInteger(int) = int {
                        frame[n] = StackValue::SignedInteger(int);
                    } else {
                        panic!("The value gethered from the stack is not an signed integer")
                    }
                } else {
                    panic!("Something went wrong seems like the stack is empty")
                }
            }

            OpCodes::iload_(n) => {
                if let StackValue::SignedInteger(int) = frame[n] {
                    stack.push(StackValue::SignedInteger(int));
                    frame[n] = StackValue::None;
                }
            }

            OpCodes::iconst_(n) => {
                stack.push(StackValue::SignedInteger(n));
            }

            OpCodes::fstore_(n) => {
                if let Some(float) = stack.pop() {
                    if let StackValue::Float(float) = float {
                        frame[n] = StackValue::Float(float);
                    } else {
                        panic!("The value gathered from the stack is not a float")
                    }
                } else {
                    panic!("Oops seems like the stack is empty. this wasn't supposed to happen")
                }
            }

            OpCodes::fload_(n) => {
                if let StackValue::Float(float) = frame[n] {
                    stack.push(StackValue::Float(float));
                } else {
                    panic!("The frame value at index ( {} ) is either empty or doesn't contain float value", n);
                }
            }

            OpCodes::fconst_(float) => {
                stack.push(StackValue::Float(float));
            }

            // Return void (do nothing)
            OpCodes::Return | OpCodes::nop => {}

            // Handle the opcodes that isn't implemented yet
            OpCodes::OpCodeError => {
                println!("Remaining byte code: {:?}", bytes);
                panic!(
                    "The OpCode( {} ), isn't implemented or doesn't exist",
                    opcode_byte
                );
            }
            #[allow(unreachable_patterns)]
            unknown_opcode => {
                panic!("The OpCode( {} : {:?} )", opcode_byte, unknown_opcode)
            }
        }
    }
}

*/

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
