use std::collections::VecDeque;

use jvm_parser::{
    self,
    attributes::CodeAttribute,
    content_pool::CpInfo,
    utils::{read_u1, read_u2, read_u4},
    ClassFile,
};

#[allow(non_camel_case_types)]
#[derive(Debug)]
pub enum OpCodes {
    OpCodeError = 0xfd,
    nop = 0x00,
    getstatic = 0xb2,
    ldc = 0x12,
    invokevirtual = 0xb6,
    bipush = 0x10,
    istore_1 = 0x3c,
    iconst_4 = 0x7,
    iload_1 = 0x1b,

    Return = 0xb1,
}

impl From<u8> for OpCodes {
    fn from(v: u8) -> Self {
        match v {
            0x00 => OpCodes::nop,
            0xb2 => OpCodes::getstatic,
            0x12 => OpCodes::ldc,
            0xb6 => OpCodes::invokevirtual,
            0x10 => OpCodes::bipush,
            0x3c => OpCodes::istore_1,
            0x7 => OpCodes::iconst_4,
            0x1b => OpCodes::iload_1,

            0xb1 => OpCodes::Return,
            _ => OpCodes::OpCodeError,
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
enum StackValue {
    Integer(u32),
    Float(f32),
    String(String),
    Byte(u8),
    ObjectRef(StackObjectRef),
    Invalid,
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

    let mut frame: VecDeque<StackValue> = vec![].into();

    while bytes.len() > 0 {
        let opcode = read_u1(&mut bytes);
        match OpCodes::from(opcode) {
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
                    _ => {}
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
                // println!("{:#?}", bytes);
                // TODO: Verify that it should be u8 or i8
                let byte = read_u1(&mut bytes);
                stack.push_back(StackValue::Byte(byte));
            }

            OpCodes::istore_1 => {
                let byte = read_u1(&mut bytes);
                stack.push_back(StackValue::Byte(byte))
            }

            OpCodes::iconst_4 => stack.push_back(StackValue::Byte(4)),
            OpCodes::iload_1 => {}

            // Return void (do nothing)
            OpCodes::Return | OpCodes::nop => {}

            // Handle the opcodes that isn't implemented yet
            OpCodes::OpCodeError => {
                println!("Remaining byte code: {:?}", bytes);
                panic!(
                    "The OpCode( {} ), isn't implemented or doesn't exist",
                    opcode
                );
            }
            #[allow(unreachable_patterns)]
            unknown_opcode => {
                panic!("The OpCode( {} : {:?} )", opcode, unknown_opcode)
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
