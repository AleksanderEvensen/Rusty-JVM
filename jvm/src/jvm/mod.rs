pub mod opcodes;
pub mod traits;

use jvm_parser::{attributes::CodeAttribute, utils::read_u1, ClassFile, MethodInfo};
use opcodes::OpCodes;

use self::traits::{JavaClass, JavaClassInitContext};

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
                _ => {}
            }
        }
    }
}
