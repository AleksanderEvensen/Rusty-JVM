use crate::utils::{read_bytes, read_u1, read_u2, read_u4};

#[derive(Debug, Default)]
pub struct ConstantPool(pub Vec<CpInfo>);

impl ConstantPool {
    pub fn from_bytes(bytes: &mut Vec<u8>, pool_count: &u16) -> Self {
        let mut pool = ConstantPool(vec![]);

        for _ in 0..(pool_count - 1) {
            let cp_info = match read_u1(bytes) {
                7 => CpInfo::Class(CpInfoClass {
                    tag: "CONSTANT_Class".to_string(),
                    name_index: read_u2(bytes),
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
                    let bytes = read_bytes(bytes, 4);

                    let mut byte_array: [u8; 4] = Default::default();

                    bytes
                        .iter()
                        .enumerate()
                        .for_each(|(i, byte)| byte_array[i] = byte.clone());

                    println!("{:?}", byte_array);
                    CpInfo::Integrer(CpInfoInteger {
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
                    panic!("Unknown CONSTANT_TYPE: {}", unknown_tag)
                }
            };
            // println!("cp_info = {:?}", cp_info);
            pool.0.push(cp_info);
        }
        pool
    }

    pub fn get_class_at(&self, index: u16) -> Option<&CpInfoClass> {
        if let CpInfo::Class(class) = &self.0[index as usize - 1] {
            return Some(class);
        }
        None
    }

    pub fn get_refs_at(&self, index: u16) -> Option<&CpInfoRefs> {
        if let CpInfo::Refs(refs) = &self.0[index as usize - 1] {
            return Some(refs);
        }
        None
    }

    pub fn get_name_type_at(&self, index: u16) -> Option<&CpInfoNameAndType> {
        if let CpInfo::NameAndType(name_type) = &self.0[index as usize - 1] {
            return Some(name_type);
        }
        None
    }

    pub fn get_utf8_at(&self, index: u16) -> Option<&CpInfoUtf8> {
        if let CpInfo::Utf8(utf8) = &self.0[index as usize - 1] {
            return Some(utf8);
        }
        None
    }

    pub fn get_string_at(&self, index: u16) -> Option<&CpInfoString> {
        if let CpInfo::String(str) = &self.0[index as usize - 1] {
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
    Integrer(CpInfoInteger),
    Float(CpInfoFloat),
    Long(CpInfoLong),
    Double(CpInfoDouble),
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
