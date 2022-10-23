use std::{error::Error, path::PathBuf};

// From: https://docs.oracle.com/javase/specs/jvms/se7/html/jvms-4.html#jvms-4.1
#[derive(Debug, Default)]
pub struct ClassFile {
    pub magic: u32,
    pub minor_version: u16,
    pub major_version: u16,
    pub constant_pool_count: u16,
    pub constant_pool: Vec<CpInfo>,
    pub access_flags: AccessFlags,
    pub this_class: u16,
    pub super_class: u16,
    pub interfaces_count: u16,
    pub interfaces: Vec<Interfaces>,
    pub fields_count: u16,
    pub fields: Vec<FieldInfo>,
    pub methods_count: u16,
    pub methods: Vec<Methods_MethodInfo>,
    pub attributes_count: u16,
    pub attributes: Vec<AttributeInfo>,
}

impl ClassFile {
    pub fn parse_constant_pool(mut bytes: &mut Vec<u8>, pool_count: &u16) -> Vec<CpInfo> {
        let mut pool = vec![];

        for _ in 0..(pool_count - 1) {
            let cp_info = match read_u1(&mut bytes) {
                7 => CpInfo::Class(CpInfoClass {
                    tag: "CONSTANT_Class".to_string(),
                    name_index: read_u2(&mut bytes),
                }),

                9 => CpInfo::Refs(CpInfoRefs {
                    tag: "CONSTANT_FieldRef".to_string(),
                    class_index: read_u2(bytes),
                    name_and_type_index: read_u2(bytes),
                }),

                10 => CpInfo::Refs(CpInfoRefs {
                    tag: "CONSTANT_MethodRef".to_string(),
                    class_index: read_u2(bytes),
                    name_and_type_index: read_u2(bytes),
                }),

                11 => CpInfo::Refs(CpInfoRefs {
                    tag: "CONSTANT_InterfaceMethodref".to_string(),
                    class_index: read_u2(bytes),
                    name_and_type_index: read_u2(bytes),
                }),

                8 => CpInfo::String(CpInfoString {
                    tag: "CONSTANT_String".to_string(),
                    string_index: read_u2(bytes),
                }),

                3 => {
                    todo!("CONSTANT_Integer not implemented")
                }

                4 => {
                    todo!("CONSTANT_Float not implemented")
                }

                5 => {
                    todo!("CONSTANT_Long not implemented")
                }

                6 => {
                    todo!("CONSTANT_Double not implemented")
                }

                12 => CpInfo::NameAndType(CpInfoNameAndType {
                    tag: "CONSTANT_NameAndType".to_string(),
                    name_index: read_u2(bytes),
                    descriptor_index: read_u2(bytes),
                }),

                1 => {
                    let length = read_u2(bytes);

                    let utf8_bytes = bytes.drain(0..(length as usize)).collect::<Vec<u8>>();
                    let text = String::from_utf8(utf8_bytes.clone()).unwrap();

                    CpInfo::Utf8(CpInfoUtf8 {
                        tag: "CONSTANT_Utf8".to_string(),
                        length: length,
                        bytes: utf8_bytes,
                        data: text,
                    })
                }

                15 => {
                    todo!("CONSTANT_MethodHandle not implemented")
                }

                16 => {
                    todo!("CONSTANT_MethodType not implemented")
                }

                18 => {
                    todo!("CONSTANT_InvokeDynamic not implemented")
                }

                unknown_tag => {
                    todo!("Unknown CONSTANT_TYPE: {}", unknown_tag)
                }
            };
            println!("cp_info = {:?}", cp_info);
            pool.push(cp_info);
        }
        pool
    }

    pub fn parse_methods(
        bytes: &mut Vec<u8>,
        method_count: &u16,
        constant_pool: &Vec<CpInfo>,
    ) -> Vec<Methods_MethodInfo> {
        let mut methods = vec![];

        for _ in 0..*method_count as usize {
            let valid_masks = parse_flags(
                read_u2(bytes),
                vec![
                    0x0001, 0x0002, 0x0004, 0x0008, 0x0010, 0x0020, 0x0040, 0x0080, 0x0100, 0x0400,
                    0x0800, 0x1000,
                ],
            );
            let mut method = Methods_MethodInfo {
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
            let attrib_name_index = read_u2(bytes);
            let attrib_length = read_u4(bytes);

            let pool_entry = constant_pool
                .get(attrib_name_index as usize - 1)
                .expect("Failed to parse methods, index out of CONSTANT Pool bounds");

            let attribute_type = match pool_entry {
                CpInfo::Utf8(utf8_data) => match utf8_data.data.as_str() {
                    "Code" => {
                        let max_stack = read_u2(bytes);
                        let max_locals = read_u2(bytes);
                        let code_length = read_u4(bytes);
                        let code = bytes.drain(0..code_length as usize).collect();
                        let exception_table_length = read_u2(bytes);

                        let mut exception_table = vec![];
                        for _ in 0..exception_table_length {
                            exception_table.push(ExceptionTable {
                                start_pc: read_u2(bytes),
                                end_pc: read_u2(bytes),
                                handler_pc: read_u2(bytes),
                                catch_type: read_u2(bytes),
                            });
                        }

                        let attributes_count = read_u2(bytes);
                        let attribute_info =
                            ClassFile::parse_attributes(bytes, attribute_count, constant_pool);
                        AttributeInfoTypes::Code {
                            max_stack,
                            max_locals,
                            code_length,
                            code,
                            exception_table_length,
                            exception_table,
                            attributes_count,
                            attribute_info,
                        }
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

                        AttributeInfoTypes::LineNumberTable {
                            line_number_table_length,
                            line_number_table,
                        }
                    }

                    "SourceFile" => AttributeInfoTypes::SourceFile {
                        sourcefile_index: read_u2(bytes),
                    },

                    not_implemented_type => {
                        todo!(
                            "Implement attribute parsing for attribute: {}",
                            not_implemented_type
                        )
                    }
                },

                wrong_type => {
                    panic!("(Method Parsing) The pool entry at ATTRIBUTE_NAME_INDEX should be CONSTANT_Utf8, not {:#?}", wrong_type)
                }
            };

            attributes.push(AttributeInfo {
                attribute_name_index: attrib_name_index,
                attribute_length: attrib_length,
                attibutes: attribute_type,
            })
        }

        attributes
    }
}

impl ClassFile {
    pub fn get_class_from_pool(&self, index: u16) -> Option<&CpInfoClass> {
        if let CpInfo::Class(class) = &self.constant_pool[index as usize - 1] {
            return Some(class);
        }
        None
    }

    pub fn get_refs_from_pool(&self, index: u16) -> Option<&CpInfoRefs> {
        if let CpInfo::Refs(refs) = &self.constant_pool[index as usize - 1] {
            return Some(refs);
        }
        None
    }

    pub fn get_name_and_type_from_pool(&self, index: u16) -> Option<&CpInfoNameAndType> {
        if let CpInfo::NameAndType(name_type) = &self.constant_pool[index as usize - 1] {
            return Some(name_type);
        }
        None
    }

    pub fn get_utf8_from_pool(&self, index: u16) -> Option<&CpInfoUtf8> {
        if let CpInfo::Utf8(utf8) = &self.constant_pool[index as usize - 1] {
            return Some(utf8);
        }
        None
    }

    pub fn get_string_from_pool(&self, index: u16) -> Option<&CpInfoString> {
        if let CpInfo::String(str) = &self.constant_pool[index as usize - 1] {
            return Some(str);
        }
        None
    }

    pub fn get_refs_from_pool_ext(
        &self,
        index: u16,
    ) -> Option<(&CpInfoRefs, &CpInfoClass, &CpInfoNameAndType)> {
        if let Some(refs) = self.get_refs_from_pool(index) {
            if let Some(class) = self.get_class_from_pool(refs.class_index) {
                if let Some(name_type) = self.get_name_and_type_from_pool(refs.name_and_type_index)
                {
                    return Some((refs, class, name_type));
                }
            }
        }

        return None;
    }
}

#[derive(Debug, Default)]
pub struct AccessFlags {
    pub flags: Vec<String>,
    pub byte_flags: Vec<u16>,
}

#[derive(Debug, Default)]
pub struct Methods_MethodInfo {
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

// From: https://docs.oracle.com/javase/specs/jvms/se7/html/jvms-4.html#jvms-4.4
#[derive(Debug, Default, Clone)]
pub enum CpInfo {
    #[default]
    None,

    Class(CpInfoClass),
    Refs(CpInfoRefs),
    NameAndType(CpInfoNameAndType),
    Utf8(CpInfoUtf8),
    String(CpInfoString),
}

#[derive(Debug, Default, Clone)]
pub struct CpInfoClass {
    pub tag: String,
    pub name_index: u16,
}
#[derive(Debug, Default, Clone)]
pub struct CpInfoRefs {
    pub tag: String,
    pub class_index: u16,
    pub name_and_type_index: u16,
}
#[derive(Debug, Default, Clone)]
pub struct CpInfoNameAndType {
    pub tag: String,
    pub name_index: u16,
    pub descriptor_index: u16,
}
#[derive(Debug, Default, Clone)]
pub struct CpInfoUtf8 {
    pub tag: String,
    pub length: u16,
    pub bytes: Vec<u8>,
    pub data: String,
}
#[derive(Debug, Default, Clone)]
pub struct CpInfoString {
    pub tag: String,
    pub string_index: u16,
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
#[derive(Debug, Default)]
pub enum MethodInfo {
    #[default]
    V,
}

#[derive(Debug, Default)]
pub struct AttributeInfo {
    pub attribute_name_index: u16,
    pub attribute_length: u32,
    pub attibutes: AttributeInfoTypes,
}

#[derive(Debug, Default)]
pub enum AttributeInfoTypes {
    #[default]
    None,

    Code {
        max_stack: u16,
        max_locals: u16,
        code_length: u32,
        code: Vec<u8>,
        exception_table_length: u16,
        exception_table: Vec<ExceptionTable>,
        attributes_count: u16,
        attribute_info: Vec<AttributeInfo>,
    },

    LineNumberTable {
        line_number_table_length: u16,
        line_number_table: Vec<LineNumber>,
    },

    SourceFile {
        sourcefile_index: u16,
    },
}

#[derive(Debug, Default)]
pub struct LineNumber {
    pub start_pc: u16,
    pub line_number: u16,
}

#[derive(Debug, Default)]
pub struct ExceptionTable {
    pub start_pc: u16,
    pub end_pc: u16,
    pub handler_pc: u16,
    pub catch_type: u16,
}

pub fn parse_java_class_file(path: PathBuf) -> Result<ClassFile, Box<dyn Error>> {
    parse_java_class(std::fs::read(path)?)
}

pub fn read_u4(bytes: &mut Vec<u8>) -> u32 {
    let mut read_bytes: [u8; 4] = Default::default();

    bytes.drain(0..4).enumerate().for_each(|(i, byte)| {
        read_bytes[i] = byte;
    });
    u32::from_be_bytes(read_bytes)
}

fn get_u4(bytes: &Vec<u8>) -> u32 {
    let mut read_bytes: [u8; 4] = Default::default();
    bytes
        .get(0..4)
        .unwrap()
        .iter()
        .enumerate()
        .for_each(|(i, byte)| read_bytes[i] = byte.clone());
    u32::from_be_bytes(read_bytes)
}

pub fn read_u2(bytes: &mut Vec<u8>) -> u16 {
    let mut read_bytes: [u8; 2] = Default::default();

    bytes.drain(0..2).enumerate().for_each(|(i, byte)| {
        read_bytes[i] = byte;
    });
    u16::from_be_bytes(read_bytes)
}

fn get_u2(bytes: &Vec<u8>) -> u16 {
    let mut read_bytes: [u8; 2] = Default::default();
    bytes
        .get(0..2)
        .unwrap()
        .iter()
        .enumerate()
        .for_each(|(i, byte)| read_bytes[i] = byte.clone());
    u16::from_be_bytes(read_bytes)
}

pub fn read_u1(bytes: &mut Vec<u8>) -> u8 {
    bytes.remove(0)
}

pub fn parse_java_class(mut bytes: Vec<u8>) -> Result<ClassFile, Box<dyn Error>> {
    let mut class_file = ClassFile {
        magic: read_u4(&mut bytes),
        minor_version: read_u2(&mut bytes),
        major_version: read_u2(&mut bytes),
        ..Default::default()
    };

    // Read Constant Pool
    class_file.constant_pool_count = read_u2(&mut bytes);
    class_file.constant_pool =
        ClassFile::parse_constant_pool(&mut bytes, &class_file.constant_pool_count);

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
    class_file.interfaces_count = read_u2(&mut bytes);

    class_file.interfaces = vec![]; // TODO: implement interface parsing

    class_file.fields_count = read_u2(&mut bytes);
    class_file.fields = vec![]; // TODO: implement field parsing

    class_file.methods_count = read_u2(&mut bytes);
    class_file.methods = ClassFile::parse_methods(
        &mut bytes,
        &class_file.methods_count,
        &class_file.constant_pool,
    );

    class_file.attributes_count = read_u2(&mut bytes);
    class_file.attributes = ClassFile::parse_attributes(
        &mut bytes,
        &class_file.attributes_count,
        &class_file.constant_pool,
    );

    Ok(class_file)
}
