pub mod opcodes;
pub mod traits;

use jvm_parser::{
    attributes::CodeAttribute,
    content_pool::CpInfo,
    utils::{read_u1, read_u2},
    ClassFile, MethodInfo,
};
use opcodes::OpCodes;

pub enum DebugLevel {
    None = 0,
    Log = 1,
    Debug = 2,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
enum StackValue {
    Integer(i32),
    Float(f32),
    String(String),
    Short(i16),
    Byte(u8),
    JavaObjectRef(JavaObjectRef),
    Invalid,
    #[default]
    None,
}

#[derive(Debug, Clone)]
pub struct JavaObjectRef {
    index: u8,
    class_name: String,
}

pub struct JVM {
    debug_level: u8,
    class_file: ClassFile,
}

impl JVM {
    pub fn new(class_file: ClassFile) -> Self {
        Self {
            class_file,
            debug_level: DebugLevel::None as u8,
        }
    }

    pub fn set_debug_level(&mut self, debug_level: DebugLevel) {
        self.debug_level = debug_level as u8;
    }

    pub fn run_main(&self) -> Result<(), String> {
        if let Some((method, code_attribute)) = self.class_file.get_main_method() {
            self.execute_code(method, code_attribute);
        } else {
            return Err("Class File doesn't contain main method".to_string());
        }

        return Ok(());
    }

    pub fn execute_code(&self, method: &MethodInfo, code_data: CodeAttribute) {
        let mut bytes = code_data.code;

        let mut java_objects: Vec<String> = vec![];
        let mut operand_stack: Vec<StackValue> = vec![];
        let mut frame = vec![StackValue::default(); code_data.max_locals as usize];

        while bytes.len() > 0 {
            let opcode_byte = read_u1(&mut bytes);
            let opcode = OpCodes::from(opcode_byte);

            match opcode {
                OpCodes::getstatic => {
                    let index = read_u2(&mut bytes);

                    let (refs, class, name_type) = self
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
                    let name_type_name = self
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

                    operand_stack.push(StackValue::JavaObjectRef(JavaObjectRef {
                        index: 0,
                        class_name: class_name,
                    }));
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
                    todo!("OpCode new")
                }

                OpCodes::dup => {
                    let last_element = operand_stack.last().unwrap().clone();
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

                _ => {}
            }
        }
    }
}
