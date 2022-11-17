use crate::utils::{read_bytes, read_u1, read_u2, read_u4};

#[derive(Debug, Clone)]
pub struct ConstantPool {
    pub pool_entries: Vec<CpInfo>,
}

impl ConstantPool {
    pub fn from_bytes(bytes: &mut Vec<u8>) -> Self {
        let pool_count = read_u2(bytes);
        let mut pool: Vec<CpInfo> = vec![];

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
                }),

                16 => {
                    todo!("CONSTANT_MethodType not implemented")
                }

                18 => CpInfo::InvokeDynamic(CpInfoInvokeDynamic {
                    tag: "CONSTANT_InvokeDynamic".to_string(),
                    bootstrap_method_attr_index: read_u2(bytes),
                    name_and_type_index: read_u2(bytes),
                }),
                unknown_tag => {
                    panic!("Unknown CONSTANT_TYPE: {}", unknown_tag)
                }
            };
            pool.push(cp_info);
        }
        ConstantPool { pool_entries: pool }
    }

    pub fn get_at(&self, index: u16) -> Option<&CpInfo> {
        self.pool_entries.get(index as usize - 1)
    }

    pub fn get_class_at(&self, index: u16) -> Option<&CpInfoClass> {
        if let CpInfo::Class(class) = self.get_at(index).unwrap() {
            return Some(class);
        }
        None
    }

    pub fn get_refs_at(&self, index: u16) -> Option<&CpInfoRefs> {
        if let CpInfo::Refs(refs) = self.get_at(index).unwrap() {
            return Some(refs);
        }
        None
    }

    pub fn get_name_type_at(&self, index: u16) -> Option<&CpInfoNameAndType> {
        if let CpInfo::NameAndType(name_type) = self.get_at(index).unwrap() {
            return Some(name_type);
        }
        None
    }

    pub fn get_utf8_at(&self, index: u16) -> Option<&CpInfoUtf8> {
        if let CpInfo::Utf8(utf8) = self.get_at(index).unwrap() {
            return Some(utf8);
        }
        None
    }

    pub fn get_string_at(&self, index: u16) -> Option<&CpInfoString> {
        if let CpInfo::String(str) = self.get_at(index).unwrap() {
            return Some(str);
        }
        None
    }
}

// From: https://docs.oracle.com/javase/specs/jvms/se7/html/jvms-4.html#jvms-4.4
#[derive(Debug, Clone)]
pub enum CpInfo {
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

#[derive(Debug, Clone)]
pub struct CpInfoInteger {
    pub tag: String,
    pub bytes: i32,
}
#[derive(Debug, Clone)]
pub struct CpInfoFloat {
    pub tag: String,
    pub bytes: f32,
}

#[derive(Debug, Clone)]
pub struct CpInfoLong {
    pub tag: String,
    pub high_bytes: u32,
    pub low_bytes: u32,
    pub bytes: u64,
}
#[derive(Debug, Clone)]
pub struct CpInfoDouble {
    pub tag: String,
    pub high_bytes: u32,
    pub low_bytes: u32,
    pub bytes: f64,
}

#[derive(Debug, Clone)]
pub struct CpInfoClass {
    pub tag: String,
    pub name_index: u16,
}

#[derive(Debug, Clone)]
pub struct CpInfoRefs {
    pub tag: String,
    pub class_index: u16,
    pub name_and_type_index: u16,
}

#[derive(Debug, Clone)]
pub struct CpInfoNameAndType {
    pub tag: String,
    pub name_index: u16,
    pub descriptor_index: u16,
}

#[derive(Debug, Clone)]
pub struct CpInfoUtf8 {
    pub tag: String,
    pub data: String,
}
#[derive(Debug, Clone)]
pub struct CpInfoString {
    pub tag: String,
    pub string_index: u16,
}

#[derive(Debug, Clone)]
pub struct CpInfoInvokeDynamic {
    pub tag: String,
    pub bootstrap_method_attr_index: u16,
    pub name_and_type_index: u16,
}

#[derive(Debug, Clone)]
pub struct CpInfoMethodHandle {
    pub tag: String,
    pub reference_kind: u8,
    pub reference_index: u16,
}
