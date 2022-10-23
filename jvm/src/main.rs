use jvm_parser::{
    self, read_u1, read_u2, AttributeInfo, AttributeInfoTypes, ClassFile, CpInfo,
    Methods_MethodInfo,
};

fn find_function_by_name(class_file: &ClassFile, name: String) -> Option<&Methods_MethodInfo> {
    class_file.methods.iter().find(|v| {
        match class_file.constant_pool[v.name_index as usize - 1].clone() {
            CpInfo::Utf8(utf8) => utf8.data == name,
            _ => false,
        }
    })
}

fn find_attributes_by_name<'a>(
    class_file: &'a ClassFile,
    attributes: &'a Vec<AttributeInfo>,
    name: String,
) -> Option<&'a AttributeInfo> {
    attributes.iter().find(|v| {
        match class_file.constant_pool[v.attribute_name_index as usize - 1].clone() {
            CpInfo::Utf8(utf8) => utf8.data == name,
            _ => false,
        }
    })
}

#[allow(non_camel_case_types)]
#[derive(Debug)]
pub enum OpCodes {
    OpCodeError = 0xfd,
    nop = 0x00,
    getstatic = 0xb2,
    ldc = 0x12,
    invokevirtual = 0xb6,
    Return = 0xb1,
}

impl From<u8> for OpCodes {
    fn from(v: u8) -> Self {
        match v {
            0x00 => OpCodes::nop,
            0xb2 => OpCodes::getstatic,
            0x12 => OpCodes::ldc,
            0xb6 => OpCodes::invokevirtual,
            0xb1 => OpCodes::Return,
            _ => OpCodes::OpCodeError,
        }
    }
}

#[allow(dead_code)]
#[derive(Debug)]
enum StackValue {
    Integer(u32),
    Float(f32),
    String(String),
    ObjectRef(StackObjectRef),
    Invalid,
}

#[allow(dead_code)]
#[derive(Debug)]
struct StackObjectRef {
    class_name: String,
    member_name: String,
    descriptor: String,
}

fn execute_code(class_file: &ClassFile, code: Vec<u8>) {
    println!("{:?}", code);

    let mut bytes = code;

    let mut stack: Vec<StackValue> = vec![];

    while bytes.len() > 0 {
        let opcode = read_u1(&mut bytes);
        match OpCodes::from(opcode) {
            OpCodes::getstatic => {
                let index = read_u2(&mut bytes);

                let (_, class, name_type) = class_file
                    .get_refs_from_pool_ext(index)
                    .expect("Failed to fetch data from constant pool");

                let class_name = class_file
                    .get_utf8_from_pool(class.name_index)
                    .expect("Class Name not in const pool")
                    .data
                    .clone();
                let member_name = class_file
                    .get_utf8_from_pool(name_type.name_index)
                    .expect("Member Name not in const pool")
                    .data
                    .clone();

                let descriptor = class_file
                    .get_utf8_from_pool(name_type.descriptor_index)
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

                match &class_file.constant_pool[index as usize - 1] {
                    CpInfo::String(str) => {
                        match &class_file.constant_pool[str.string_index as usize - 1] {
                            CpInfo::Utf8(utf8) => stack.push(StackValue::String(utf8.data.clone())),
                            _ => {}
                        }
                    }
                    _ => {}
                }
            }

            OpCodes::invokevirtual => {
                let index = read_u2(&mut bytes);

                let refs = class_file.get_refs_from_pool(index).unwrap();
                let class = class_file.get_class_from_pool(refs.class_index).unwrap();
                let class_name = class_file.get_utf8_from_pool(class.name_index).unwrap();
                let name_type = class_file
                    .get_name_and_type_from_pool(refs.name_and_type_index)
                    .unwrap();

                let name_type_name = class_file.get_utf8_from_pool(name_type.name_index).unwrap();
                let descriptor = class_file
                    .get_utf8_from_pool(name_type.descriptor_index)
                    .unwrap();

                if let StackValue::ObjectRef(obj_ref) = stack.drain(0..1).next().unwrap() {}

                if class_name.data == "java/io/PrintStream" {
                    match name_type_name.data.as_str() {
                        "println" => match stack.drain(0..1).next().unwrap() {
                            StackValue::Float(v) => {
                                println!("{}", v)
                            }
                            StackValue::Integer(v) => {
                                println!("{}", v)
                            }
                            StackValue::String(v) => {
                                println!("{}", v)
                            }
                            invalid_data => {
                                panic!("Invalid data on the stack, {:#?}", invalid_data);
                            }
                        },
                        "print" => match stack.drain(0..1).next().unwrap() {
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

            // Return void (do nothing)
            OpCodes::Return | OpCodes::nop => {}

            // Handle the opcodes that isn't implemented yet
            OpCodes::OpCodeError => {
                panic!(
                    "The OpCode( {} ), isn't implemented or doesn't exist",
                    opcode
                );
            }
            unknown_opcode => {
                panic!("The OpCode( {} : {:?} )", opcode, unknown_opcode)
            }
        }
    }
}

fn main() {
    let class_file: ClassFile =
        jvm_parser::parse_java_class_file("./java/MyProgram.class".into()).unwrap();

    if let Some(main_method) = find_function_by_name(&class_file, "main".to_string()) {
        // println!("{:#?}", main_method);
        if let Some(code) =
            find_attributes_by_name(&class_file, &main_method.attributes, "Code".to_string())
        {
            match &code.attibutes {
                AttributeInfoTypes::Code {
                    max_stack: _,
                    max_locals: _,
                    code_length: _,
                    code,
                    exception_table_length: _,
                    exception_table: _,
                    attributes_count: _,
                    attribute_info: _,
                } => execute_code(&class_file, code.clone()),
                _ => {}
            }
        }
    }

    println!("Hello, world, from Rust!");
}
