use std::sync::Arc;

use crate::{
    attributes::{AttributeInfo, BootstrapMethodsAttribute},
    utils::{read_bytes, read_u1, read_u2, read_u4},
    ClassFile, MethodInfo,
};

#[derive(Debug, Default, Clone)]
pub struct ConstantPool {
    pub pool_entries: Vec<CpInfo>,
}

impl ConstantPool {
    pub fn from_bytes(bytes: &mut Vec<u8>, pool_count: &u16) -> Self {
        let mut pool = ConstantPool {
            pool_entries: vec![],
        };

        for _ in 0..(pool_count - 1) {
            let cp_info = match read_u1(bytes) {
                7 => CpInfo::Class(CpInfoClass {
                    tag: "CONSTANT_Class".to_string(),
                    name_index: read_u2(bytes),
                    ..Default::default()
                }),

                9 => CpInfo::Refs(CpInfoRefs {
                    tag: "CONSTANT_FieldRef".to_string(),
                    class_index: read_u2(bytes),
                    name_and_type_index: read_u2(bytes),
                    ..Default::default()
                }),

                10 => CpInfo::Refs(CpInfoRefs {
                    tag: "CONSTANT_MethodRef".to_string(),
                    class_index: read_u2(bytes),
                    name_and_type_index: read_u2(bytes),
                    ..Default::default()
                }),

                11 => CpInfo::Refs(CpInfoRefs {
                    tag: "CONSTANT_InterfaceMethodref".to_string(),
                    class_index: read_u2(bytes),
                    name_and_type_index: read_u2(bytes),
                    ..Default::default()
                }),

                8 => CpInfo::String(CpInfoString {
                    tag: "CONSTANT_String".to_string(),
                    string_index: read_u2(bytes),
                    ..Default::default()
                }),

                3 => {
                    let bytes = read_bytes(bytes, 4);

                    let mut byte_array: [u8; 4] = Default::default();

                    bytes
                        .iter()
                        .enumerate()
                        .for_each(|(i, byte)| byte_array[i] = byte.clone());

                    println!("{:?}", byte_array);
                    println!("Hello World, Why isn't this executed");
                    CpInfo::Integer(CpInfoInteger {
                        tag: "CONSTANT_Integer".to_string(),
                        bytes: i32::from_be_bytes(byte_array),
                    })
                }

                4 => CpInfo::Float(CpInfoFloat {
                    tag: "CONSTANT_Float".to_string(),
                    bytes: f32::from_bits(read_u4(bytes)),
                }),

                5 => {
                    let high = read_u4(bytes);
                    let low = read_u4(bytes);
                    let bytes: u64 = ((high as u64) << 32) + low as u64;

                    CpInfo::Long(CpInfoLong {
                        tag: "CONSTANT_Long".to_string(),
                        high_bytes: high,
                        low_bytes: low,
                        bytes: bytes,
                    })
                }

                6 => {
                    let high = read_u4(bytes);
                    let low = read_u4(bytes);
                    let bytes: u64 = ((high as u64) << 32) + low as u64;
                    let bytes = f64::from_bits(bytes);
                    CpInfo::Double(CpInfoDouble {
                        tag: "CONSTANT_Double".to_string(),
                        high_bytes: high,
                        low_bytes: low,
                        bytes: bytes,
                    })
                }

                12 => CpInfo::NameAndType(CpInfoNameAndType {
                    tag: "CONSTANT_NameAndType".to_string(),
                    name_index: read_u2(bytes),
                    descriptor_index: read_u2(bytes),
                    ..Default::default()
                }),

                1 => {
                    let length = read_u2(bytes);
                    let text =
                        String::from_utf8(bytes.drain(0..(length as usize)).collect::<Vec<u8>>())
                            .unwrap();

                    CpInfo::Utf8(CpInfoUtf8 {
                        tag: "CONSTANT_Utf8".to_string(),
                        data: text,
                    })
                }

                15 => CpInfo::MethodHandle(CpInfoMethodHandle {
                    tag: "CONSTANT_MethodHandle".to_string(),
                    reference_kind: read_u1(bytes),
                    reference_index: read_u2(bytes),
                    ..Default::default()
                }),

                16 => {
                    todo!("CONSTANT_MethodType not implemented")
                }

                18 => CpInfo::InvokeDynamic(CpInfoInvokeDynamic {
                    tag: "CONSTANT_InvokeDynamic".to_string(),
                    bootstrap_method_attr_index: read_u2(bytes),
                    name_and_type_index: read_u2(bytes),
                    ..Default::default()
                }),
                unknown_tag => {
                    panic!("Unknown CONSTANT_TYPE: {}", unknown_tag)
                }
            };
            pool.pool_entries.push(cp_info);
        }

        pool
    }

    pub fn build_constant_pool(pool: &mut ConstantPool, class_file: &ClassFile) {
        pool.pool_entries.iter_mut().for_each(|entry| match entry {
            CpInfo::Class(class) => class.build(pool),
            CpInfo::InvokeDynamic(invoke_dyn) => invoke_dyn.build(pool),
            CpInfo::MethodHandle(method_handle) => method_handle.build(pool),
            CpInfo::NameAndType(name_and_type) => name_and_type.build(pool),
            CpInfo::Refs(refs) => refs.build(pool),
            CpInfo::String(string) => string.build(pool),
            _ => {}
        });

        for entry in pool.pool_entries.iter_mut() {}
    }

    pub fn get_at(&self, index: u16) -> &CpInfo {
        &self.pool_entries[index as usize - 1]
    }

    pub fn get_class_at(&self, index: u16) -> Option<&CpInfoClass> {
        if let CpInfo::Class(class) = &self.get_at(index) {
            return Some(class);
        }
        None
    }

    pub fn get_refs_at(&self, index: u16) -> Option<&CpInfoRefs> {
        if let CpInfo::Refs(refs) = &self.get_at(index) {
            return Some(refs);
        }
        None
    }

    pub fn get_name_type_at(&self, index: u16) -> Option<&CpInfoNameAndType> {
        if let CpInfo::NameAndType(name_type) = &self.get_at(index) {
            return Some(name_type);
        }
        None
    }

    pub fn get_utf8_at(&self, index: u16) -> Option<&CpInfoUtf8> {
        if let CpInfo::Utf8(utf8) = &self.get_at(index) {
            return Some(utf8);
        }
        None
    }

    pub fn get_string_at(&self, index: u16) -> Option<&CpInfoString> {
        if let CpInfo::String(str) = &self.get_at(index) {
            return Some(str);
        }
        None
    }

    pub fn get_refs_ext_at(
        &self,
        index: u16,
    ) -> Option<(&CpInfoRefs, &CpInfoClass, &CpInfoNameAndType)> {
        if let Some(refs) = self.get_refs_at(index) {
            if let Some(class) = self.get_class_at(refs.class_index) {
                if let Some(name_type) = self.get_name_type_at(refs.name_and_type_index) {
                    return Some((refs, class, name_type));
                }
            }
        }

        return None;
    }
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
    Integer(CpInfoInteger),
    Float(CpInfoFloat),
    Long(CpInfoLong),
    Double(CpInfoDouble),
    InvokeDynamic(CpInfoInvokeDynamic),
    MethodHandle(CpInfoMethodHandle),
}

#[derive(Debug, Default, Clone)]
pub struct CpInfoInteger {
    pub tag: String,
    pub bytes: i32,
}
#[derive(Debug, Default, Clone)]
pub struct CpInfoFloat {
    pub tag: String,
    pub bytes: f32,
}

#[derive(Debug, Default, Clone)]
pub struct CpInfoLong {
    pub tag: String,
    pub high_bytes: u32,
    pub low_bytes: u32,
    pub bytes: u64,
}
#[derive(Debug, Default, Clone)]
pub struct CpInfoDouble {
    pub tag: String,
    pub high_bytes: u32,
    pub low_bytes: u32,
    pub bytes: f64,
}

#[derive(Debug, Default, Clone)]
pub struct CpInfoClass {
    pub tag: String,
    pub name_index: u16,
    pub name: Option<String>,
}

impl CpInfoClass {
    pub fn build(&mut self, pool: &ConstantPool) {
        self.name = Some(pool.get_utf8_at(self.name_index).unwrap().clone().data);
    }
}

#[derive(Debug, Default, Clone)]
pub struct CpInfoRefs {
    pub tag: String,
    pub class_index: u16,
    pub name_and_type_index: u16,
    pub class: Option<CpInfoClass>,
    pub name_and_type: Option<CpInfoNameAndType>,
}
impl CpInfoRefs {
    pub fn build(&mut self, pool: &ConstantPool) {
        let class = pool.get_class_at(self.class_index).unwrap().clone();
        class.build(pool);
        let name_and_type = pool
            .get_name_type_at(self.name_and_type_index)
            .unwrap()
            .clone();
        name_and_type.build(pool);

        self.class = Some(class);
        self.name_and_type = Some(name_and_type);
    }
}

#[derive(Debug, Default, Clone)]
pub struct CpInfoNameAndType {
    pub tag: String,
    pub name_index: u16,
    pub descriptor_index: u16,
    pub name: Option<String>,
    pub descriptor: Option<String>,
}
impl CpInfoNameAndType {
    pub fn build(&mut self, pool: &ConstantPool) {
        self.name = Some(pool.get_utf8_at(self.name_index).unwrap().data.clone());
        self.descriptor = Some(
            pool.get_utf8_at(self.descriptor_index)
                .unwrap()
                .data
                .clone(),
        );
    }
}

#[derive(Debug, Default, Clone)]
pub struct CpInfoUtf8 {
    pub tag: String,
    pub data: String,
}
#[derive(Debug, Default, Clone)]
pub struct CpInfoString {
    pub tag: String,
    pub string_index: u16,
    pub string: Option<String>,
}
impl CpInfoString {
    pub fn build(&mut self, pool: &ConstantPool) {
        self.string = Some(pool.get_utf8_at(self.string_index).unwrap().data.clone());
    }
}

#[derive(Debug, Default, Clone)]
pub struct CpInfoInvokeDynamic {
    pub tag: String,
    pub bootstrap_method_attr_index: u16,
    pub name_and_type_index: u16,
    pub bootstrap_method_attr: Option<BootstrapMethodsAttribute>,
    pub name_and_type: Option<CpInfoNameAndType>,
}

impl CpInfoInvokeDynamic {
    pub fn build(&mut self, pool: &ConstantPool) {
        let name_and_type = pool
            .get_name_type_at(self.name_and_type_index)
            .unwrap()
            .clone();
        name_and_type.build(pool);

        self.name_and_type = Some(name_and_type);
    }
}

#[derive(Debug, Default, Clone)]
pub struct CpInfoMethodHandle {
    pub tag: String,
    pub reference_kind: u8,
    pub reference_index: u16,
    pub reference: Option<CpInfoRefs>,
}

impl CpInfoMethodHandle {
    pub fn build(&mut self, pool: &ConstantPool) {
        let refs = pool.get_refs_at(self.reference_index).unwrap().clone();
        refs.build(pool);

        self.reference = Some(refs);
    }
}
