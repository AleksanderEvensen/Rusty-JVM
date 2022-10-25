pub mod attributes;
pub mod content_pool;
pub mod utils;

use attributes::{
    AttributeInfo, AttributeInfoData, CodeAttribute, ExceptionTable, LineNumber,
    LineNumberTableAttribute, SourceFileAttribute,
};
use content_pool::{ConstantPool, CpInfo};
use std::{error::Error, path::PathBuf};
use utils::{read_bytes, read_u2, read_u4};

// From: https://docs.oracle.com/javase/specs/jvms/se7/html/jvms-4.html#jvms-4.1
#[derive(Debug, Default)]
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
    pub fn from_file(path: PathBuf) -> Result<ClassFile, Box<dyn Error>> {
        ClassFile::from_bytes(std::fs::read(path)?)
    }
    pub fn from_bytes(mut bytes: Vec<u8>) -> Result<ClassFile, Box<dyn Error>> {
        let mut class_file = ClassFile {
            magic: read_u4(&mut bytes),
            minor_version: read_u2(&mut bytes),
            major_version: read_u2(&mut bytes),
            ..Default::default()
        };

        // Read Constant Pool
        let constant_pool_count = read_u2(&mut bytes);
        class_file.constant_pool = ConstantPool::from_bytes(&mut bytes, &constant_pool_count);

        // class_file.access_flags =
        let valid_masks = parse_flags(
            read_u2(&mut bytes),
            vec![
                0x001, 0x0010, 0x0020, 0x0200, 0x0400, 0x1000, 0x2000, 0x4000,
            ],
        );

        class_file.access_flags = AccessFlags {
            byte_flags: valid_masks.clone(),
            flags: vec![
                ("ACC_PUBLIC".to_string(), 0x0001),
                ("ACC_FINAL".to_string(), 0x0010),
                ("ACC_SUPER".to_string(), 0x0020),
                ("ACC_INTERFACE".to_string(), 0x0200),
                ("ACC_ABSTRACT".to_string(), 0x0400),
                ("ACC_SYNTHETIC".to_string(), 0x1000),
                ("ACC_ANNOTATION".to_string(), 0x2000),
                ("ACC_ENUM".to_string(), 0x4000),
            ]
            .iter()
            .filter(|flag| valid_masks.contains(&flag.1))
            .map(|flag| flag.0.clone())
            .collect::<Vec<String>>(),
        };

        class_file.this_class = read_u2(&mut bytes);
        class_file.super_class = read_u2(&mut bytes);

        #[allow(unused_variables)] // TODO: Remove this when implemented
        let interfaces_count = read_u2(&mut bytes);

        class_file.interfaces = vec![]; // TODO: implement interface parsing

        #[allow(unused_variables)] // TODO: Remove this when implemented
        let fields_count = read_u2(&mut bytes);
        class_file.fields = vec![]; // TODO: implement field parsing

        let methods_count = read_u2(&mut bytes);
        class_file.methods =
            ClassFile::parse_methods(&mut bytes, &methods_count, &class_file.constant_pool.0);

        let attributes_count = read_u2(&mut bytes);
        class_file.attributes =
            ClassFile::parse_attributes(&mut bytes, &attributes_count, &class_file.constant_pool.0);

        Ok(class_file)
    }

    pub fn parse_methods(
        bytes: &mut Vec<u8>,
        method_count: &u16,
        constant_pool: &Vec<CpInfo>,
    ) -> Vec<MethodInfo> {
        let mut methods = vec![];

        for _ in 0..*method_count as usize {
            let valid_masks = parse_flags(
                read_u2(bytes),
                vec![
                    0x0001, 0x0002, 0x0004, 0x0008, 0x0010, 0x0020, 0x0040, 0x0080, 0x0100, 0x0400,
                    0x0800, 0x1000,
                ],
            );
            let mut method = MethodInfo {
                access_flags: AccessFlags {
                    byte_flags: valid_masks.clone(),
                    flags: vec![
                        ("ACC_PUBLIC".to_string(), 0x0001),
                        ("ACC_PRIVATE".to_string(), 0x0002),
                        ("ACC_PROTECTED".to_string(), 0x0004),
                        ("ACC_STATIC".to_string(), 0x0008),
                        ("ACC_FINAL".to_string(), 0x0010),
                        ("ACC_SYNCHRONIZED".to_string(), 0x0020),
                        ("ACC_BRIDGE".to_string(), 0x0040),
                        ("ACC_VARARGS".to_string(), 0x0080),
                        ("ACC_NATIVE".to_string(), 0x0100),
                        ("ACC_ABSTRACT".to_string(), 0x0400),
                        ("ACC_STRICT".to_string(), 0x0800),
                        ("ACC_SYNTHETIC".to_string(), 0x1000),
                    ]
                    .iter()
                    .filter(|flag| valid_masks.contains(&flag.1))
                    .map(|flag| flag.0.clone())
                    .collect::<Vec<String>>(),
                },
                name_index: read_u2(bytes),
                descriptor_index: read_u2(bytes),
                attributes_count: read_u2(bytes),
                attributes: vec![],
            };
            method.attributes =
                ClassFile::parse_attributes(bytes, &method.attributes_count, constant_pool);
            methods.push(method);
        }

        methods
    }

    pub fn parse_attributes(
        bytes: &mut Vec<u8>,
        attribute_count: &u16,
        constant_pool: &Vec<CpInfo>,
    ) -> Vec<AttributeInfo> {
        let mut attributes = vec![];

        for _ in 0..attribute_count.to_owned() as usize {
            let attribute_name_index = read_u2(bytes);
            let _ = read_u4(bytes);

            if let CpInfo::Utf8(utf8) = &constant_pool[attribute_name_index as usize - 1] {
                let attribute = match utf8.data.as_str() {
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

                        let attribute_count = read_u2(bytes);
                        let attribute_info =
                            ClassFile::parse_attributes(bytes, &attribute_count, constant_pool);

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
        }

        attributes
    }
}

impl ClassFile {
    pub fn get_main_method(&self) -> Option<(&MethodInfo, CodeAttribute)> {
        if let Some(method) = self.methods.iter().find(|&v| {
            if let Some(name) = &self.constant_pool.get_utf8_at(v.name_index) {
                if name.data.as_str() == "main" {
                    true
                } else {
                    false
                }
            } else {
                false
            }
        }) {
            let mut code: Option<CodeAttribute> = None;

            for attribute in method.attributes.iter() {
                if let AttributeInfoData::Code(data) = &attribute.attribute {
                    code = Some(data.clone())
                }
            }

            let code = code.unwrap();

            return Some((method, code));
        }

        None
    }
}

#[derive(Debug, Default)]
pub struct AccessFlags {
    pub flags: Vec<String>,
    pub byte_flags: Vec<u16>,
}

#[derive(Debug, Default)]
pub struct MethodInfo {
    pub access_flags: AccessFlags,
    pub name_index: u16,
    pub descriptor_index: u16,
    pub attributes_count: u16,
    pub attributes: Vec<AttributeInfo>,
}

fn parse_flags(value: u16, masks: Vec<u16>) -> Vec<u16> {
    let mut valid_masks = vec![];
    for mask in masks.iter() {
        if (value & mask.to_owned()) != 0 {
            valid_masks.push(mask.clone())
        }
    }
    valid_masks
}

#[derive(Debug, Default)]
pub enum Interfaces {
    #[default]
    V,
}
#[derive(Debug, Default)]
pub enum FieldInfo {
    #[default]
    V,
}
