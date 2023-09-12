pub mod opcodes;

use std::collections::HashMap;

use jvm_parser::{
    classfile::{
        attributes::{AttributeInfoData, CodeAttribute},
        classfile::{MethodAccessFlags, MethodInfo},
        constant_pool::CpInfo,
        JavaClass,
    },
    jar::JarFile,
};

use crate::{
    jvm::opcodes::OpCodes,
    utils::{parse_descriptor, Descriptor, DescriptorTypes},
};

use self::opcodes::parse_opcodes;
// use jvm_parser::ClassFile;

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
    classes: HashMap<String, JavaClass>,
    main_method_class: Option<String>,
    native_methods: HashMap<&'static str, Box<dyn Fn(Vec<StackValue>, Descriptor) -> StackValue>>,
}

impl JVM {
    pub fn new() -> Self {
        let mut natives: HashMap<
            &'static str,
            Box<dyn Fn(Vec<StackValue>, Descriptor) -> StackValue>,
        > = HashMap::new();
        natives.insert(
            "com/ahse/jvm/Main;print",
            Box::new(|args, descriptor| match descriptor {
                Descriptor {
                    return_value: DescriptorTypes::Void,
                    parameters: params,
                } if params.get(0)
                    == Some(&DescriptorTypes::Class("java/lang/String".to_string())) =>
                {
                    let StackValue::String(v) = &args[0] else {
                        panic!(
                            "A string wans't passed as an argument to the print(String) function"
                        );
                    };
                    println!("{v}");
                    StackValue::None
                }

                _ => {
                    todo!("Implement this");
                }
            }),
        );

        Self {
            classes: HashMap::new(),
            main_method_class: None,
            native_methods: natives,
        }
    }

    pub fn add_class(&mut self, java_class: JavaClass) -> Result<(), String> {
        let Some(cp_class) = java_class.constant_pool.get_class_at(java_class.this_class) else {
            return Err(format!(
                "No class constant pool entry at location {}, found: {:?}",
                java_class.this_class,
                java_class.constant_pool.get_at(java_class.this_class)
            ));
        };

        let Some(class_name) = java_class.constant_pool.get_utf8_at(cp_class.name_index) else {
            return Err(format!(
                "No Utf8 constant pool entry at location {}, found: {:?}",
                cp_class.name_index,
                java_class.constant_pool.get_at(cp_class.name_index)
            ));
        };

        if java_class.access_flags & 0x0001 != 0 {
            if let Some(method) = java_class.get_method_by_name(&"main".to_string()) {
                if method.access_flags == (0x0001 | 0x0008) {
                    self.main_method_class = Some(class_name.data.clone());
                }
            }
        }
        self.classes.insert(class_name.data.clone(), java_class);

        Ok(())
    }

    pub fn add_jar(&mut self, jar_file: JarFile) -> Result<(), String> {
        for (file_name, java_class) in jar_file.classes {
            if let Err(error_msg) = self.add_class(java_class) {
                return Err(format!(
                    "Failed to add the class file: {file_name}\n{error_msg}"
                ));
            }
        }

        Ok(())
    }

    pub fn get_classes(&self) -> &HashMap<String, JavaClass> {
        return &self.classes;
    }

    fn get_method_from_class(
        &self,
        class_name: &str,
        method_name: &str,
    ) -> Option<(&JavaClass, &MethodInfo)> {
        let Some(class) = &self.classes.get(class_name) else {
            return None;
        };

        let Some(method) = class.get_method_by_name(&method_name.to_string()) else {
            return None;
        };

        Some((class, method))
    }

    pub fn run(&self) -> Result<(), String> {
        let Some(main_method_class_name) = &self.main_method_class else {
            return Err("There is no main function to run".to_string());
        };

        let Some((main_class, method)) = self.get_method_from_class(main_method_class_name, "main")
        else {
            return Err(format!(
                "Could not find main method in class: {main_method_class_name}"
            ));
        };

        if method.access_flags != /* public static */ (0x0001 | 0x0008) {
            return Err(format!(
                "The main method in the class '{main_method_class_name}' is not public and static"
            ));
        }

        let Some(method_byte_code) = method
            .attributes
            .iter()
            .find(|attribute| match attribute.attribute {
                AttributeInfoData::Code(_) => true,
                _ => false,
            })
            .map(|attribute| match &attribute.attribute {
                AttributeInfoData::Code(byte_code_data) => byte_code_data,
                _ => unreachable!(),
            })
        else {
            return Err(format!("The main method in the class '{main_method_class_name}' does not have runnable byte code"));
        };

        // dbg!(method_byte_code);

        self.execute_code(method, method_byte_code, main_class);

        Ok(())
    }

    pub fn execute_code(
        &self,
        _method: &MethodInfo,
        code_data: &CodeAttribute,
        java_class: &JavaClass,
    ) {
        let opcodes = parse_opcodes(&code_data.code).unwrap();
        let mut stack: Vec<StackValue> = vec![];

        for opcode in opcodes {
            #[allow(unused_variables)]
            match opcode {
                OpCodes::dup => todo!(),
                OpCodes::getstatic(cp_index) => todo!(),
                OpCodes::ldc(cp_index) => {
                    let Some(entry) = java_class.constant_pool.get_at(cp_index as u16) else {
                        panic!("No entry at index: {cp_index} in constant_pool");
                    };

                    match entry {
                        CpInfo::Integer(cp_int) => todo!("ldc integer"),
                        CpInfo::Float(cp_f) => todo!("ldc float"),
                        CpInfo::Class(cp_class) => todo!("ldc class"),
                        CpInfo::String(cp_str) => stack.push(StackValue::String(
                            java_class
                                .constant_pool
                                .get_utf8_at(cp_str.string_index)
                                .unwrap()
                                .data
                                .clone(),
                        )),
                        CpInfo::MethodHandle(cp_method) => todo!("ldc method handle"),
                        CpInfo::MethodType(cp_method_type) => todo!("ldc method type"),
                        // TODO: CpInfo::Dynamic -- find out which this one is
                        _ => panic!("Tried to load an unloadable constant value"),
                    }
                }
                OpCodes::ldc2_w(cp_index) => todo!(),
                OpCodes::ldc_w(cp_index) => todo!(),
                OpCodes::invokevirtual(cp_index) => todo!(),
                OpCodes::invokespecial(cp_index) => todo!(),
                OpCodes::invokestatic(cp_index) => {
                    let Some((_, cp_class, cp_name_n_type)) =
                        java_class.constant_pool.get_refs_ext_at(cp_index as u16)
                    else {
                        panic!("Didn't find shit");
                    };

                    let Some(class_name) =
                        java_class.constant_pool.get_utf8_at(cp_class.name_index)
                    else {
                        panic!(
                            "Couldn't find class name in constant pool at index: {}",
                            cp_class.name_index
                        );
                    };

                    let Some(method_name) = java_class
                        .constant_pool
                        .get_utf8_at(cp_name_n_type.name_index)
                    else {
                        panic!(
                            "Couldn't find method name in constant pool at index: {}",
                            cp_name_n_type.name_index
                        );
                    };

                    let Some((j_class, method)) =
                        self.get_method_from_class(&class_name.data, &method_name.data)
                    else {
                        panic!(
                            "Couldn't find class '{}' with method '{}' in class hash list",
                            class_name.data, method_name.data
                        );
                    };

                    if method.access_flags & MethodAccessFlags::ACC_NATIVE != 0 {
                        let Some(native_method) = self
                            .native_methods
                            .get(format!("{};{}", class_name.data, method_name.data).as_str())
                        else {
                            panic!(
                                "Native method isn't in library: {};{}",
                                class_name.data, method_name.data
                            );
                        };

                        let Some(descriptor) =
                            j_class.constant_pool.get_utf8_at(method.descriptor_index)
                        else {
                            panic!("Failed to find descriptor");
                        };

                        let descriptor = parse_descriptor(&descriptor.data);

                        let params = stack
                            .drain(stack.len().saturating_sub(descriptor.parameters.len())..)
                            .collect::<Vec<_>>();

                        native_method(params, descriptor);
                    }

                    // println!("{:#?}",);
                }
                OpCodes::bipush(byte) => todo!(),
                OpCodes::sipush(short) => todo!(),
                OpCodes::new(cp_index) => todo!(),
                OpCodes::anewarray(cp_index) => todo!(),
                OpCodes::istore_(local_index) => todo!(),
                OpCodes::iconst_(local_index) => todo!(),
                OpCodes::iload_(local_index) => todo!(),
                OpCodes::fstore_(local_index) => todo!(),
                OpCodes::fconst_(local_index) => todo!(),
                OpCodes::fload_(local_index) => todo!(),
                OpCodes::astore_(local_index) => todo!(),
                OpCodes::aconst_null => todo!(),
                OpCodes::aload_(local_index) => todo!(),

                OpCodes::nop | OpCodes::Return => {}

                _ => {}
            }
        }

        // let mut reader = ByteReader::from_vec(&code_data.code);
        // reader.set_endian(byte_reader::Endian::Big);

        // let mut static_classes: HashMap<String, JavaClass> = HashMap::new();
        // let mut java_objects: Vec<JavaClass> = vec![];
        // let mut operand_stack: Vec<StackValue> = vec![];
        // let mut frame = vec![StackValue::default(); code_data.max_locals as usize];

        // #[allow(unused)]
        // fn debug_memory(
        //     operand_stack: &Vec<StackValue>,
        //     frame: &Vec<StackValue>,
        //     java_objects: &Vec<JavaClass>,
        //     static_classes: &HashMap<String, JavaClass>,
        // ) {
        //     println!("==================\nCurrent JVM Memory:");
        //     println!("Operand Stack: {:#?}\n", operand_stack);
        //     println!("Current Frame: {:#?}\n", frame);
        //     println!("Java Object Size: {}", java_objects.len());
        //     println!("Java Static Classes: {:#?}", static_classes.keys());
        // }

        // #[allow(unused)]
        // fn debug_bytecode(bytecode: &Vec<u8>) {
        //     println!("===========================\nCurrent Remaining bytecode:");
        //     println!("Byte Code Bytes: {:?}", &bytecode);
        //     println!("With potential opcodes:");

        //     let mut iter = bytecode.iter();

        //     while let Some(v) = iter.next() {
        //         let opcode = OpCodes::from(*v);
        //         match opcode {
        //             OpCodes::getstatic => {
        //                 let index =
        //                     u16::from_be_bytes([*iter.next().unwrap(), *iter.next().unwrap()]);
        //                 println!("  {:?}({})", opcode, index);
        //             }
        //             OpCodes::ldc => {
        //                 let index = u8::from_be_bytes([*iter.next().unwrap()]);
        //                 println!("  {:?}({})", opcode, index);
        //             }

        //             OpCodes::invokevirtual => {
        //                 let index =
        //                     u16::from_be_bytes([*iter.next().unwrap(), *iter.next().unwrap()]);
        //                 println!("  {:?}({})", opcode, index);
        //             }

        //             OpCodes::invokestatic => {
        //                 let index =
        //                     u16::from_be_bytes([*iter.next().unwrap(), *iter.next().unwrap()]);
        //                 println!("  {:?}({})", opcode, index);
        //             }

        //             v => println!("  {:?}", v),
        //         }
        //     }
        //     println!();
        // }

        // debug_bytecode(&reader.peak_rest().unwrap());
        // while let Ok(opcode_byte) = reader.read::<u8>() {
        //     let opcode_byte = opcode_byte;
        //     let opcode = OpCodes::from(opcode_byte);
        //     println!("[Executing Opcode : {opcode:?}]");

        //     match opcode {
        //         OpCodes::ldc => {
        //             let index: u8 = reader.read().unwrap();

        //             let stack_value = match java_class.constant_pool.get_at(index as u16).unwrap() {
        //                 CpInfo::String(str) => {
        //                     let text = java_class
        //                         .constant_pool
        //                         .get_utf8_at(str.string_index)
        //                         .unwrap()
        //                         .data
        //                         .clone();

        //                     StackValue::String(text)
        //                 }

        //                 CpInfo::Integer(int) => StackValue::Integer(int.bytes),
        //                 CpInfo::Float(float) => StackValue::Float(float.bytes),

        //                 unimplemented_type => {
        //                     panic!("[OpCode : LDC] The value at index ( {} ) in the constant_pool, does not have an implementation on the operand stack. Constant pool value: {:#?}", index, unimplemented_type);
        //                 }
        //             };
        //             operand_stack.push(stack_value);
        //         }

        //         OpCodes::invokestatic => {
        //             let index: u16 = reader.read().unwrap();

        //             let (_, class, name_type) =
        //                 java_class.constant_pool.get_refs_ext_at(index).unwrap();
        //             let class_name = java_class
        //                 .constant_pool
        //                 .get_utf8_at(class.name_index)
        //                 .unwrap()
        //                 .data
        //                 .clone();

        //             let function_name = java_class
        //                 .constant_pool
        //                 .get_utf8_at(name_type.name_index)
        //                 .unwrap()
        //                 .data
        //                 .clone();
        //             let descriptor = java_class
        //                 .constant_pool
        //                 .get_utf8_at(name_type.descriptor_index)
        //                 .unwrap()
        //                 .data
        //                 .clone();

        //             let descriptor = parse_descriptor(&descriptor);

        //             let target_class = self.classes.get(&class_name).unwrap();

        //             let method = target_class.get_method_by_name(&function_name).unwrap();

        //             // match method.access_flags {}

        //             if match_flag(
        //                 method.access_flags,
        //                 MethodAccessFlags::ACC_NATIVE | MethodAccessFlags::ACC_SYNCHRONIZED,
        //             ) {
        //                 todo!("Implement the seperate logic when executing native and synchronized: https://docs.oracle.com/javase/specs/jvms/se19/html/jvms-6.html#jvms-6.5.invokestatic")
        //             } else if method.access_flags & MethodAccessFlags::ACC_NATIVE as u16 != 0 {
        //                 // println!("[Invokenative]");

        //                 self.native_methods
        //                     .get(format!("{class_name};{function_name}").as_str())
        //                     .unwrap()(
        //                     vec![StackValue::String("Hello World, Hardcoded".to_string())],
        //                     descriptor,
        //                 );
        //             } else {
        //                 todo!("Implement logic for invoking static functions for a specified class")
        //             }

        //             // println!(
        //             //     "[OpCodes : invokestatic] Invoking {:#?}",
        //             //     (&class_name, &function_name, &descriptor)
        //             // );
        //         }

        //         OpCodes::Return => { /* for now, just don't do shit */ }
        //         OpCodes::nop => { /* this is meant to be empty */ }
        //         unknown_opcode => {
        //             panic!(
        //                 "The OpCode ( {} : {:?} ) is not imeplemented",
        //                 opcode_byte, unknown_opcode
        //             )
        //         }
        //     }
        // }

        // while bytes.len() > 0 {
        //     match opcode {
        //         OpCodes::getstatic => {
        //             let index = read_u2(&mut bytes);

        //             let (_, class, name_type) = self
        //                 .class_file
        //                 .constant_pool
        //                 .get_refs_ext_at(index)
        //                 .unwrap();

        //             let class_name = self
        //                 .class_file
        //                 .constant_pool
        //                 .get_utf8_at(class.name_index)
        //                 .unwrap()
        //                 .data
        //                 .clone();
        //             let _name_type_name = self
        //                 .class_file
        //                 .constant_pool
        //                 .get_utf8_at(name_type.name_index)
        //                 .unwrap()
        //                 .data
        //                 .clone();
        //             let _descriptor = self
        //                 .class_file
        //                 .constant_pool
        //                 .get_utf8_at(name_type.descriptor_index)
        //                 .unwrap()
        //                 .data
        //                 .clone();

        //             dbgprint!(
        //                 "[OpCodes : getstatic] Initialize new: {:#?}",
        //                 (&class_name, &_name_type_name, &_descriptor)
        //             );

        //             if !static_classes.contains_key(&class_name) {
        //                 let class =
        //                     get_class_constructor(&class_name)(JavaClassInitContext::empty());
        //                 static_classes.insert(String::from(&class_name), class);
        //             }

        //             if let Some(_) = static_classes.get(&class_name) {
        //                 operand_stack
        //                     .push(StackValue::JavaStaticClassRef(String::from(&class_name)));
        //             }
        //         }

        //         OpCodes::ldc => {
        //             let index = read_u1(&mut bytes);
        //             let value = match &self.class_file.constant_pool.get_at(index as u16) {
        //                 CpInfo::String(str) => {
        //                     let text = self
        //                         .class_file
        //                         .constant_pool
        //                         .get_utf8_at(str.string_index)
        //                         .unwrap()
        //                         .data
        //                         .clone();

        //                     StackValue::String(text)
        //                 }

        //                 CpInfo::Integer(int) => StackValue::Integer(int.bytes),
        //                 CpInfo::Float(float) => StackValue::Float(float.bytes),

        //                 unimplemented_type => {
        //                     panic!("[OpCode : LDC] The value at index ( {} ) in the constant_pool, does not have an implementation on the operand stack. Constant pool value: {:#?}", index, unimplemented_type);
        //                 }
        //             };

        //             dbgprint!("[OpCode : LDC] Adding value to operand_stack: {:?}", value);
        //             operand_stack.push(value);
        //         }

        //         OpCodes::invokevirtual => {
        //             let index = read_u2(&mut bytes);
        //             let (_, class, name_type) = self
        //                 .class_file
        //                 .constant_pool
        //                 .get_refs_ext_at(index)
        //                 .unwrap();

        //             let class_name = self
        //                 .class_file
        //                 .constant_pool
        //                 .get_utf8_at(class.name_index)
        //                 .unwrap();
        //             let name_type_name = self
        //                 .class_file
        //                 .constant_pool
        //                 .get_utf8_at(name_type.name_index)
        //                 .unwrap();
        //             let descriptor = self
        //                 .class_file
        //                 .constant_pool
        //                 .get_utf8_at(name_type.descriptor_index)
        //                 .unwrap();

        //             dbgprint!(
        //                 "[OpCode : invokevirtual] Invoking: {:#?}",
        //                 (&class_name.data, &name_type_name.data, &descriptor.data)
        //             );

        //             dbgprint!("Operand Stack: {:#?}", operand_stack);

        //             todo!("OpCode invokevirtual")
        //         }

        //         OpCodes::invokespecial => {
        //             let index = read_u2(&mut bytes);

        //             let (_refs, class, name_type) = self
        //                 .class_file
        //                 .constant_pool
        //                 .get_refs_ext_at(index)
        //                 .unwrap();

        //             let class_name = self
        //                 .class_file
        //                 .constant_pool
        //                 .get_utf8_at(class.name_index)
        //                 .unwrap()
        //                 .data
        //                 .clone();

        //             let name_type_name = self
        //                 .class_file
        //                 .constant_pool
        //                 .get_utf8_at(name_type.name_index)
        //                 .unwrap()
        //                 .data
        //                 .clone();
        //             let descriptor = self
        //                 .class_file
        //                 .constant_pool
        //                 .get_utf8_at(name_type.descriptor_index)
        //                 .unwrap()
        //                 .data
        //                 .clone();

        //             let descriptor = parse_descriptor(&descriptor);

        //             dbgprint!(
        //                 "[OpCodes : invokespecial] Invoking {:#?}",
        //                 (&class_name, &name_type_name, &descriptor)
        //             );

        //             if name_type_name == "<init>" {
        //                 let java_class =
        //                     get_class_constructor(&class_name)(JavaClassInitContext::empty());
        //             } else {
        //             }

        //             todo!("OpCode invokespecial")
        //         }

        //         OpCodes::invokestatic => {
        //             let index = read_u2(&mut bytes);

        //             let (_, class, name_type) = self
        //                 .class_file
        //                 .constant_pool
        //                 .get_refs_ext_at(index)
        //                 .unwrap();
        //             let class_name = self
        //                 .class_file
        //                 .constant_pool
        //                 .get_utf8_at(class.name_index)
        //                 .unwrap()
        //                 .data
        //                 .clone();

        //             let name_type_name = self
        //                 .class_file
        //                 .constant_pool
        //                 .get_utf8_at(name_type.name_index)
        //                 .unwrap()
        //                 .data
        //                 .clone();
        //             let descriptor = self
        //                 .class_file
        //                 .constant_pool
        //                 .get_utf8_at(name_type.descriptor_index)
        //                 .unwrap()
        //                 .data
        //                 .clone();

        //             let descriptor = parse_descriptor(&descriptor);

        //             dbgprint!(
        //                 "[OpCodes : invokestatic] Invoking {:#?}",
        //                 (&class_name, &name_type_name, &descriptor)
        //             );
        //         }

        //         OpCodes::bipush => {
        //             let byte = read_u1(&mut bytes);
        //             operand_stack.push(StackValue::Integer(i32::from_be_bytes([0, 0, 0, byte])));
        //         }

        //         OpCodes::sipush => {
        //             let bytes = read_u2(&mut bytes);
        //             operand_stack.push(StackValue::Short(i16::from_be_bytes(bytes.to_be_bytes())));
        //         }

        //         OpCodes::new => {
        //             let index = read_u2(&mut bytes);

        //             let class = self.class_file.constant_pool.get_class_at(index).unwrap();
        //             let class_name = self
        //                 .class_file
        //                 .constant_pool
        //                 .get_utf8_at(class.name_index)
        //                 .unwrap()
        //                 .data
        //                 .clone();

        //             dbgprint!(
        //                 "[OpCodes : new] Initialize new class: {:#?}",
        //                 (&class, &class_name)
        //             );

        //             let java_class =
        //                 get_class_constructor(&class_name)(JavaClassInitContext::empty());
        //             java_objects.push(java_class);

        //             operand_stack.push(StackValue::JavaObjectRef(JavaObjectRef {
        //                 index: java_objects.len() - 1,
        //             }));
        //         }

        //         OpCodes::dup => {
        //             let last_element = operand_stack.last().unwrap().clone();
        //             dbgprint!("[OpCodes : dup] Duplicate {:#?}", last_element);
        //             operand_stack.push(last_element);
        //         }

        //         OpCodes::istore_(n) => {
        //             if let Some(int) = operand_stack.pop() {
        //                 if let StackValue::Integer(int) = int {
        //                     frame[n] = StackValue::Integer(int);
        //                 } else {
        //                     panic!("[OpCode : istore_{}] The stack value that was poped from the operand stack w", n);
        //                 }
        //             } else {
        //                 panic!("[OpCode : istore_{}] Something wen't wrong. the operand stack seems to be empty when attempted to pop from", n);
        //             }
        //         }

        //         OpCodes::iload_(n) => {
        //             if let StackValue::Integer(int) = frame[n] {
        //                 operand_stack.push(StackValue::Integer(int));
        //                 frame[n] = StackValue::None;
        //             }
        //         }

        //         OpCodes::iconst_(n) => {
        //             operand_stack.push(StackValue::Integer(n));
        //         }

        //         OpCodes::fstore_(n) => {
        //             if let Some(float) = operand_stack.pop() {
        //                 if let StackValue::Float(float) = float {
        //                     frame[n] = StackValue::Float(float);
        //                 } else {
        //                     panic!("The value gathered from the stack is not a float")
        //                 }
        //             } else {
        //                 panic!("Oops seems like the stack is empty. this wasn't supposed to happen")
        //             }
        //         }

        //         OpCodes::fload_(n) => {
        //             if let StackValue::Float(float) = frame[n] {
        //                 operand_stack.push(StackValue::Float(float));
        //             } else {
        //                 panic!("The frame value at index ( {} ) is either empty or doesn't contain float value", n);
        //             }
        //         }

        //         OpCodes::fconst_(float) => {
        //             operand_stack.push(StackValue::Float(float));
        //         }

        //         // Return or void or do nothing
        //         OpCodes::Return | OpCodes::nop => {}

        //         // Handle the opcodes that ins't implemented
        //         OpCodes::OpCodeError(_) => {
        //             panic!(
        //                 "The OpCode ( {} ), is not implemented or doesn't exist",
        //                 opcode_byte
        //             );
        //         }
        //         #[allow(unreachable_patterns)]
        //         unknown_opcode => {
        //             panic!(
        //                 "The OpCode ( {} : {:?} ) is not imeplemented",
        //                 opcode_byte, unknown_opcode
        //             )
        //         }
        //     }
        // }
    }
}
