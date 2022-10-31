pub mod opcodes;
pub mod traits;

use std::collections::HashMap;

use jvm_parser::{
    attributes::CodeAttribute,
    constant_pool::CpInfo,
    utils::{read_u1, read_u2},
    ClassFile, MethodInfo,
};
use opcodes::OpCodes;

use crate::{
    java_mappings::get_class_constructor, jvm::traits::JavaClassInitContext,
    utils::parse_descriptor,
};

use self::traits::JavaClass;

#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
enum StackValue {
    Integer(i32),
    Float(f32),
    String(String),
    Short(i16),
    Byte(u8),
    JavaObjectRef(JavaObjectRef),
    JavaStaticClassRef(String),
    Invalid,
    #[default]
    None,
}

#[derive(Debug, Clone)]
pub struct JavaObjectRef {
    index: usize,
}

pub struct JVM {
    class_file: ClassFile,
}

impl JVM {
    pub fn new(class_file: ClassFile) -> Self {
        Self { class_file }
    }

    pub fn get_main(&self) -> Result<(&MethodInfo, CodeAttribute), String> {
        if let Some((method, code_attribute)) = self.class_file.get_main_method() {
            return Ok((method, code_attribute));
        } else {
            return Err("Class File doesn't contain main method".to_string());
        }
    }

    pub fn execute_code(&self, _method: &MethodInfo, code_data: CodeAttribute) {
        let mut bytes = code_data.code;

        let mut static_classes: HashMap<String, Box<dyn JavaClass>> = HashMap::new();
        let mut java_objects: Vec<Box<dyn JavaClass>> = vec![];
        let mut operand_stack: Vec<StackValue> = vec![];
        let mut frame = vec![StackValue::default(); code_data.max_locals as usize];

        fn debug_memory(
            operand_stack: &Vec<StackValue>,
            frame: &Vec<StackValue>,
            java_objects: &Vec<Box<dyn JavaClass>>,
            static_classes: &HashMap<String, Box<dyn JavaClass>>,
        ) {
            println!("==================\nCurrent JVM Memory:");
            println!("Operand Stack: {:#?}\n", operand_stack);
            println!("Current Frame: {:#?}\n", frame);
            println!("Java Object Size: {}", java_objects.len());
            println!("Java Static Classes: {:#?}", static_classes.keys());
        }

        while bytes.len() > 0 {
            let opcode_byte = read_u1(&mut bytes);
            let opcode = OpCodes::from(opcode_byte);

            match opcode {
                OpCodes::getstatic => {
                    let index = read_u2(&mut bytes);

                    let (_, class, name_type) = self
                        .class_file
                        .constant_pool
                        .get_refs_ext_at(index)
                        .unwrap();

                    let class_name = self
                        .class_file
                        .constant_pool
                        .get_utf8_at(class.name_index)
                        .unwrap()
                        .data
                        .clone();
                    let _name_type_name = self
                        .class_file
                        .constant_pool
                        .get_utf8_at(name_type.name_index)
                        .unwrap()
                        .data
                        .clone();
                    let _descriptor = self
                        .class_file
                        .constant_pool
                        .get_utf8_at(name_type.descriptor_index)
                        .unwrap()
                        .data
                        .clone();

                    #[cfg(feature = "debug")]
                    println!(
                        "[OpCodes : getstatic] Initialize new: {:#?}",
                        (&class_name, &_name_type_name, &_descriptor)
                    );

                    if !static_classes.contains_key(&class_name) {
                        let class = get_class_constructor(&class_name)(JavaClassInitContext {});
                        static_classes.insert(String::from(&class_name), class);
                    }

                    if let Some(_) = static_classes.get(&class_name) {
                        operand_stack
                            .push(StackValue::JavaStaticClassRef(String::from(&class_name)));
                    }
                }

                OpCodes::ldc => {
                    let index = read_u2(&mut bytes);
                    let value = match &self.class_file.constant_pool.get_at(index) {
                        CpInfo::String(str) => {
                            let text = self
                                .class_file
                                .constant_pool
                                .get_utf8_at(str.string_index)
                                .unwrap()
                                .data
                                .clone();

                            StackValue::String(text)
                        }

                        CpInfo::Integer(int) => StackValue::Integer(int.bytes),
                        CpInfo::Float(float) => StackValue::Float(float.bytes),

                        unimplemented_type => {
                            panic!("[OpCode : LDC] The value at index ( {} ) in the constant_pool, does not have an implementation on the operand stack. Constant pool value: {:#?}", index, unimplemented_type);
                        }
                    };
                    operand_stack.push(value);
                }

                OpCodes::invokevirtual => {
                    todo!("OpCode invokevirtual")
                }

                OpCodes::invokespecial => {
                    let index = read_u2(&mut bytes);

                    let (_refs, class, name_type) = self
                        .class_file
                        .constant_pool
                        .get_refs_ext_at(index)
                        .unwrap();

                    let _class_name = self
                        .class_file
                        .constant_pool
                        .get_utf8_at(class.name_index)
                        .unwrap()
                        .data
                        .clone();

                    let _name_type_name = self
                        .class_file
                        .constant_pool
                        .get_utf8_at(name_type.name_index)
                        .unwrap()
                        .data
                        .clone();
                    let descriptor = self
                        .class_file
                        .constant_pool
                        .get_utf8_at(name_type.descriptor_index)
                        .unwrap()
                        .data
                        .clone();

                    let _descriptor = parse_descriptor(&descriptor);

                    #[cfg(feature = "debug")]
                    println!(
                        "[OpCodes : invokespecial] Invoking {:#?}",
                        (&_class_name, &_name_type_name, &_descriptor)
                    );
                    todo!("OpCode invokespecial")
                }

                OpCodes::bipush => {
                    let byte = read_u1(&mut bytes);
                    operand_stack.push(StackValue::Integer(i32::from_be_bytes([0, 0, 0, byte])));
                }

                OpCodes::sipush => {
                    let bytes = read_u2(&mut bytes);
                    operand_stack.push(StackValue::Short(i16::from_be_bytes(bytes.to_be_bytes())));
                }

                OpCodes::new => {
                    let index = read_u2(&mut bytes);

                    let class = self.class_file.constant_pool.get_class_at(index).unwrap();
                    let class_name = self
                        .class_file
                        .constant_pool
                        .get_utf8_at(class.name_index)
                        .unwrap()
                        .data
                        .clone();

                    #[cfg(feature = "debug")]
                    println!(
                        "[OpCodes : new] Initialize new class: {:#?}",
                        (&class, &class_name)
                    );

                    let java_class = get_class_constructor(&class_name)(JavaClassInitContext {});
                    java_objects.push(java_class);

                    operand_stack.push(StackValue::JavaObjectRef(JavaObjectRef {
                        index: java_objects.len() - 1,
                    }));
                }

                OpCodes::dup => {
                    let last_element = operand_stack.last().unwrap().clone();
                    #[cfg(feature = "debug")]
                    println!("[OpCodes : dup] Duplicate {:#?}", last_element);
                    operand_stack.push(last_element);
                }

                OpCodes::istore_(n) => {
                    if let Some(int) = operand_stack.pop() {
                        if let StackValue::Integer(int) = int {
                            frame[n] = StackValue::Integer(int);
                        } else {
                            panic!("[OpCode : istore_{}] The stack value that was poped from the operand stack w", n);
                        }
                    } else {
                        panic!("[OpCode : istore_{}] Something wen't wrong. the operand stack seems to be empty when attempted to pop from", n);
                    }
                }

                OpCodes::iload_(n) => {
                    if let StackValue::Integer(int) = frame[n] {
                        operand_stack.push(StackValue::Integer(int));
                        frame[n] = StackValue::None;
                    }
                }

                OpCodes::iconst_(n) => {
                    operand_stack.push(StackValue::Integer(n));
                }

                OpCodes::fstore_(n) => {
                    if let Some(float) = operand_stack.pop() {
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
                        operand_stack.push(StackValue::Float(float));
                    } else {
                        panic!("The frame value at index ( {} ) is either empty or doesn't contain float value", n);
                    }
                }

                OpCodes::fconst_(float) => {
                    operand_stack.push(StackValue::Float(float));
                }

                // Return or void or do nothing
                OpCodes::Return | OpCodes::nop => {}

                // Handle the opcodes that ins't implemented
                OpCodes::OpCodeError => {
                    println!("Remaining byte code: {:?}", bytes);
                    panic!(
                        "The OpCode ( {} ), is not implemented or doesn't exist",
                        opcode_byte
                    );
                }
                #[allow(unreachable_patterns)]
                unknown_opcode => {
                    panic!(
                        "The OpCode ( {} : {:?} ) is not imeplemented",
                        opcode_byte, unknown_opcode
                    )
                }
            }
        }
    }
}
