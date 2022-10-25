use std::collections::VecDeque;

use jvm_parser::{
    self,
    attributes::CodeAttribute,
    content_pool::CpInfo,
    utils::{read_u1, read_u2},
    ClassFile,
};

#[allow(non_camel_case_types)]
#[derive(Debug)]
pub enum OpCodes {
    OpCodeError,
    nop,
    getstatic,
    ldc,
    invokevirtual,
    bipush,

    istore_(usize),
    iconst_(i32),
    iload_(usize),

    fstore_(usize),
    fconst_(f32),
    fload_(usize),

    Return,
}

impl From<u8> for OpCodes {
    fn from(v: u8) -> Self {
        match v {
            0x00 => OpCodes::nop,
            0xb2 => OpCodes::getstatic,
            0x12 => OpCodes::ldc,
            0xb6 => OpCodes::invokevirtual,
            0x10 => OpCodes::bipush,

            0x3b => OpCodes::istore_(0),
            0x3c => OpCodes::istore_(1),
            0x3d => OpCodes::istore_(2),
            0x3e => OpCodes::istore_(3),

            0x2 => OpCodes::iconst_(-1),
            0x3 => OpCodes::iconst_(0),
            0x4 => OpCodes::iconst_(1),
            0x5 => OpCodes::iconst_(2),
            0x6 => OpCodes::iconst_(3),
            0x7 => OpCodes::iconst_(4),

            0x1a => OpCodes::iload_(0),
            0x1b => OpCodes::iload_(1),
            0x1c => OpCodes::iload_(2),
            0x1d => OpCodes::iload_(3),

            0x43 => OpCodes::fstore_(0),
            0x44 => OpCodes::fstore_(1),
            0x45 => OpCodes::fstore_(2),
            0x46 => OpCodes::fstore_(3),

            0xb => OpCodes::fconst_(0.0),
            0xc => OpCodes::fconst_(1.0),
            0xd => OpCodes::fconst_(2.0),

            0x22 => OpCodes::fload_(0),
            0x23 => OpCodes::fload_(1),
            0x24 => OpCodes::fload_(2),
            0x25 => OpCodes::fload_(3),

            0xb1 => OpCodes::Return,
            _ => OpCodes::OpCodeError,
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
enum StackValue {
    Integer(u32),
    SignedInteger(i32),
    Float(f32),
    String(String),
    Byte(u8),
    ObjectRef(StackObjectRef),
    Invalid,
    #[default]
    None,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
struct StackObjectRef {
    class_name: String,
    member_name: String,
    descriptor: String,
}

fn execute_code(class_file: &ClassFile, code_data: CodeAttribute) {
    let mut bytes = code_data.code;

    let mut stack: VecDeque<StackValue> = vec![].into();

    let mut frame: VecDeque<StackValue> =
        vec![StackValue::default(); code_data.max_locals as usize].into();

    while bytes.len() > 0 {
        let opcode_byte = read_u1(&mut bytes);

        let opcode = OpCodes::from(opcode_byte);

        // println!("Running OpCode: {:?}", opcode);

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

                stack.push_back(StackValue::ObjectRef(StackObjectRef {
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
                            CpInfo::Utf8(utf8) => {
                                stack.push_back(StackValue::String(utf8.data.clone()))
                            }

                            _ => {}
                        }
                    }

                    CpInfo::Float(float) => {
                        stack.push_back(StackValue::Float(float.bytes));
                    }

                    CpInfo::Integrer(int) => {
                        stack.push_back(StackValue::SignedInteger(int.bytes));
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
                // let descriptor = class_file
                //     .get_utf8_from_pool(name_type.descriptor_index)
                //     .unwrap();

                if let StackValue::ObjectRef(_) = stack.pop_front().unwrap() {}

                if class_name.data == "java/io/PrintStream" {
                    match name_type_name.data.as_str() {
                        "println" => match stack.pop_front().unwrap() {
                            StackValue::Float(v) => {
                                println!("{}", v)
                            }
                            StackValue::Integer(v) => {
                                println!("{}", v)
                            }
                            StackValue::String(v) => {
                                println!("{}", v)
                            }

                            StackValue::Byte(v) => {
                                println!("{}", v)
                            }
                            StackValue::SignedInteger(v) => {
                                println!("{}", v)
                            }
                            invalid_data => {
                                panic!("Invalid data on the stack, {:#?}", invalid_data);
                            }
                        },
                        "print" => match stack.pop_front().unwrap() {
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
                        _ => {}
                    }
                }
            }

            OpCodes::bipush => {
                let byte = read_u1(&mut bytes);
                stack.push_back(StackValue::SignedInteger(i32::from_be_bytes([
                    0, 0, 0, byte,
                ])));
            }

            OpCodes::istore_(n) => {
                if let Some(int) = stack.pop_back() {
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
                    stack.push_back(StackValue::SignedInteger(int));
                    frame[n] = StackValue::None;
                }
            }

            OpCodes::iconst_(n) => {
                stack.push_back(StackValue::SignedInteger(n));
            }

            OpCodes::fstore_(n) => {
                if let Some(float) = stack.pop_back() {
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
                    stack.push_back(StackValue::Float(float));
                } else {
                    panic!("The frame value at index ( {} ) is either empty or doesn't contain float value", n);
                }
            }

            OpCodes::fconst_(float) => {
                stack.push_back(StackValue::Float(float));
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

fn main() {
    let class_file = ClassFile::from_file("./java/MyProgram.class".into()).unwrap();

    if let Some((method, code)) = class_file.get_main_method() {
        execute_code(&class_file, code)
    }

    println!("Hello, world, from Rust!");
}
