pub mod attributes;
pub mod constant_pool;
pub mod utils;

use attributes::{
    AttributeInfo, AttributeInfoData, BootstrapMethod, BootstrapMethodsAttribute, CodeAttribute,
    ExceptionTable, LineNumber, LineNumberTableAttribute, SourceFileAttribute,
};
use constant_pool::ConstantPool;
use std::{error::Error, path::PathBuf};
use utils::{read_bytes, read_u2, read_u4};

// From: https://docs.oracle.com/javase/specs/jvms/se7/html/jvms-4.html#jvms-4.1
#[derive(Debug)]
pub struct ClassFile {
    pub magic: u32,
    pub minor_version: u16,
    pub major_version: u16,
    pub constant_pool: ConstantPool,
    pub access_flags: AccessFlags,
    pub this_class: u16,
    pub super_class: u16,
    pub interfaces: Vec<Interfaces>,
    pub fields: Vec<FieldInfo>,
    pub methods: Vec<MethodInfo>,
    pub attributes: Vec<AttributeInfo>,
}

impl ClassFile {
    pub fn from_file(path: &PathBuf) -> Result<ClassFile, Box<dyn Error>> {
        ClassFile::from_bytes(std::fs::read(path)?)
    }
    pub fn from_bytes(mut bytes: Vec<u8>) -> Result<ClassFile, Box<dyn Error>> {
        let mut class = ClassFile {
            magic: read_u4(&mut bytes),
            minor_version: read_u2(&mut bytes),
            major_version: read_u2(&mut bytes),
            constant_pool: ConstantPool::from_bytes(&mut bytes),
            access_flags: AccessFlags {
                flags: parse_flags(
                    read_u2(&mut bytes),
                    vec![
                        (0x001, "ACC_PUBLIC".to_string()),
                        (0x0010, "ACC_FINAL".to_string()),
                        (0x0020, "ACC_SUPER".to_string()),
                        (0x0020, "ACC_INTERFACE".to_string()),
                        (0x0040, "ACC_ABSTRACT".to_string()),
                        (0x1000, "ACC_SYNTHETIC".to_string()),
                        (0x2000, "ACC_ANNOTATION".to_string()),
                        (0x4000, "ACC_ENUM".to_string()),
                    ],
                ),
            },
            this_class: read_u2(&mut bytes),
            super_class: read_u2(&mut bytes),
            interfaces: Vec::with_capacity(read_u2(&mut bytes) as usize), // TODO: Implement interface parsing from bytes
            fields: Vec::with_capacity(read_u2(&mut bytes) as usize), // TODO: Implement field parsing from bytes
            methods: vec![],
            attributes: vec![],
        };

        class.methods = ClassFile::parse_methods(&mut bytes, &class.constant_pool);

        class.attributes = ClassFile::parse_attributes(&mut bytes, &class.constant_pool);

        Ok(class)
    }

    pub fn parse_methods(bytes: &mut Vec<u8>, constant_pool: &ConstantPool) -> Vec<MethodInfo> {
        let method_count = read_u2(bytes);
        let mut methods = vec![];

        for _ in 0..method_count as usize {
            let mut method = MethodInfo {
                access_flags: AccessFlags {
                    flags: parse_flags(
                        read_u2(bytes),
                        vec![
                            (0x0001, "ACC_PUBLIC".to_string()),
                            (0x0002, "ACC_PRIVATE".to_string()),
                            (0x0004, "ACC_PROTECTED".to_string()),
                            (0x0008, "ACC_STATIC".to_string()),
                            (0x0010, "ACC_FINAL".to_string()),
                            (0x0020, "ACC_SYNCHRONIZED".to_string()),
                            (0x0040, "ACC_BRIDGE".to_string()),
                            (0x0080, "ACC_VARARGS".to_string()),
                            (0x0100, "ACC_NATIVE".to_string()),
                            (0x0400, "ACC_ABSTRACT".to_string()),
                            (0x0800, "ACC_STRICT".to_string()),
                            (0x1000, "ACC_SYNTHETIC".to_string()),
                        ],
                    ),
                },
                name_index: read_u2(bytes),
                descriptor_index: read_u2(bytes),
                attributes: vec![],
            };
            method.attributes = ClassFile::parse_attributes(bytes, constant_pool);
            methods.push(method);
        }

        methods
    }

    pub fn parse_attributes(
        bytes: &mut Vec<u8>,
        constant_pool: &ConstantPool,
    ) -> Vec<AttributeInfo> {
        let attribute_count = read_u2(bytes);
        let mut attributes = vec![];

        for _ in 0..attribute_count.to_owned() as usize {
            let attribute_name_index = read_u2(bytes);
            let _ = read_u4(bytes);

            let attribute_tag = constant_pool.get_utf8_at(attribute_name_index).unwrap();
            let attribute = match attribute_tag.data.as_str() {
                "Code" => {
                    let max_stack = read_u2(bytes);
                    let max_locals = read_u2(bytes);
                    let code_length = read_u4(bytes);
                    let code = read_bytes(bytes, code_length as usize);
                    let exception_table_length = read_u2(bytes);

                    let mut exception_table = vec![];

                    for _ in 0..exception_table_length as usize {
                        exception_table.push(ExceptionTable {
                            start_pc: read_u2(bytes),
                            end_pc: read_u2(bytes),
                            handler_pc: read_u2(bytes),
                            catch_type: read_u2(bytes),
                        });
                    }

                    let attribute_info = ClassFile::parse_attributes(bytes, constant_pool);

                    AttributeInfoData::Code(CodeAttribute {
                        max_stack,
                        max_locals,
                        code,
                        exception_table,
                        attribute_info,
                    })
                }

                "LineNumberTable" => {
                    let line_number_table_length = read_u2(bytes);
                    let mut line_number_table = vec![];

                    for _ in 0..line_number_table_length as usize {
                        line_number_table.push(LineNumber {
                            start_pc: read_u2(bytes),
                            line_number: read_u2(bytes),
                        })
                    }

                    AttributeInfoData::LineNumberTable(LineNumberTableAttribute {
                        line_number_table,
                    })
                }

                "SourceFile" => AttributeInfoData::SourceFile(SourceFileAttribute {
                    sourcefile_index: read_u2(bytes),
                }),

                "BootstrapMethods" => {
                    let attribute_name_index = read_u2(bytes);
                    #[allow(unused_variables)]
                    let attribute_length = read_u4(bytes);
                    let num_bootstrap_methods = read_u2(bytes);
                    let mut bootstrap_methods = vec![];

                    for _ in 0..num_bootstrap_methods as usize {
                        let bootstrap_method_ref = read_u2(bytes);
                        let num_bootstrap_arguments = read_u2(bytes);
                        let mut bootstrap_arguments = vec![];

                        for _ in 0..num_bootstrap_arguments as usize {
                            let arg_index = read_u2(bytes);
                            bootstrap_arguments.push(arg_index);
                        }

                        bootstrap_methods.push(BootstrapMethod {
                            bootstrap_method_ref,
                            bootstrap_arguments,
                        });
                    }

                    AttributeInfoData::BootstrapMethods(BootstrapMethodsAttribute {
                        attribute_name_index,
                        bootstrap_methods,
                    })
                }

                not_implemented_type => {
                    todo!(
                        "Implement attribute parsing for attribute: {}",
                        not_implemented_type
                    )
                }
            };
            attributes.push(AttributeInfo {
                attribute_name_index,
                attribute,
            });
        }

        attributes
    }
}
#[derive(Debug)]
pub struct AccessFlags {
    pub flags: Vec<String>,
}

#[derive(Debug)]
pub struct MethodInfo {
    pub access_flags: AccessFlags,
    pub name_index: u16,
    pub descriptor_index: u16,
    pub attributes: Vec<AttributeInfo>,
}

fn parse_flags<T>(value: u16, masks: Vec<(u16, T)>) -> Vec<T> {
    let mut matching_flags = vec![];
    for mask in masks {
        if (value & mask.0.to_owned()) != 0 {
            matching_flags.push(mask.1);
        }
    }
    return matching_flags;
}

#[derive(Debug)]
pub enum Interfaces {
    V,
}
#[derive(Debug)]
pub enum FieldInfo {
    V,
}
