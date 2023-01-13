use binary_reader::BinaryReader;
use std::io::Result;

#[derive(Debug, Clone)]
pub struct ConstantPool {
    pub pool_entries: Vec<CpInfo>,
}

impl ConstantPool {
    pub fn from_reader(reader: &mut BinaryReader) -> Result<Self> {
        let pool_count = *reader.read::<u16>()? - 1;

        let mut last_was_8byte = false;
        let entries: Vec<CpInfo> = (0..pool_count)
            .map(|i| {
                if last_was_8byte {
                    last_was_8byte = false;
                    return CpInfo::EmptyCpEntry;
                }
                let cp_tag = *reader.read::<u8>().unwrap();

                if cp_tag == 5 || cp_tag == 6 { // CONSTANT_Long CONSTANT_Double
                    last_was_8byte = true;
                }

                return match cp_tag  {
                    1 => CpInfo::Utf8(CpInfoUtf8 { tag: "CONSTANT_Utf8", data: reader.read_string_lossy_length::<u16>().unwrap() }),
                    3 => CpInfo::Integer(CpInfoInteger { tag: "CONSTANT_Integer", bytes: *reader.read().unwrap() }),
                    4 => CpInfo::Float(CpInfoFloat { tag: "CONSTANT_Float", bytes: *reader.read().unwrap() }),
                    5 => CpInfo::Long(CpInfoLong { tag: "CONSTANT_Long", bytes: *reader.read().unwrap() }),
                    6 => CpInfo::Double(CpInfoDouble { tag: "CONSTANT_Double", bytes: *reader.read().unwrap() }),
                    7 => CpInfo::Class(CpInfoClass { tag: "CONSTANT_Class", name_index: *reader.read().unwrap() }),
                    8 => CpInfo::String(CpInfoString { tag: "CONSTANT_String", string_index: *reader.read().unwrap() }),
                    9 => CpInfo::Refs(CpInfoRefs { tag: "CONSTANT_Fieldref", class_index: *reader.read().unwrap(), name_and_type_index: *reader.read().unwrap() }),
                    10 => CpInfo::Refs(CpInfoRefs { tag: "CONSTANT_Methodref", class_index: *reader.read().unwrap(), name_and_type_index: *reader.read().unwrap() }),
                    11 => CpInfo::Refs(CpInfoRefs { tag: "CONSTANT_InterfaceMethodref", class_index: *reader.read().unwrap(), name_and_type_index: *reader.read().unwrap() }),
                    12 => CpInfo::NameAndType(CpInfoNameAndType { tag: "CONSTANT_NameAndType", name_index: *reader.read().unwrap(), descriptor_index: *reader.read().unwrap() }),
                    15 => CpInfo::MethodHandle(CpInfoMethodHandle { tag: "CONSTANT_MethodHandle", reference_kind: *reader.read().unwrap(), reference_index: *reader.read().unwrap() }),
                    16 => CpInfo::MethodType(CpInfoMethodType { tag: "CONSTANT_MethodType", descriptor_index: *reader.read().unwrap() }),
                    17 => CpInfo::InvokeDynamic(CpInfoInvokeDynamic { tag: "CONSTANT_Dynamic", bootstrap_method_attr_index: *reader.read().unwrap(), name_and_type_index: *reader.read().unwrap() }),
                    18 => CpInfo::InvokeDynamic(CpInfoInvokeDynamic { tag: "CONSTANT_InvokeDynamic", bootstrap_method_attr_index: *reader.read().unwrap(), name_and_type_index: *reader.read().unwrap() }),
                    19 => todo!("Implement CONSTANT_TYPE: 'CONSTANT_Module'"),
                    20 => todo!("Implement CONSTANT_TYPE: 'CONSTANT_Package'"),

                    unknown_tag => unreachable!("Unknown CONSTANT_TYPE: {}\nConsult the oracle documentation for missing tag: https://docs.oracle.com/javase/specs/jvms/se19/html/jvms-4.html#jvms-4.4\n\nSome Extra Information:\nindex: {i}\npool_count: {pool_count}\nOffset: {:#X}\n", unknown_tag, reader.get_current_offset() - 1),
                };
            }).collect();

        Ok(Self {
            pool_entries: entries,
        })
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
    /// This dumb mf was needed because Oracle thought it was funny that 8byte Constant Pool entries shuld take up two spots
    /// Thankfully they admitted that this was a dumb mistake on their side (too late to fix apparently)
    /// Source: Check the italic text: https://docs.oracle.com/javase/specs/jvms/se19/html/jvms-4.html#jvms-4.4.5
    EmptyCpEntry,

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
    MethodType(CpInfoMethodType),
}

#[derive(Debug, Clone)]
pub struct CpInfoInteger {
    pub tag: &'static str,
    pub bytes: i32,
}
#[derive(Debug, Clone)]
pub struct CpInfoFloat {
    pub tag: &'static str,
    pub bytes: f32,
}

#[derive(Debug, Clone)]
pub struct CpInfoLong {
    pub tag: &'static str,
    pub bytes: u64,
}
#[derive(Debug, Clone)]
pub struct CpInfoDouble {
    pub tag: &'static str,
    pub bytes: f64,
}

#[derive(Debug, Clone)]
pub struct CpInfoClass {
    pub tag: &'static str,
    pub name_index: u16,
}

#[derive(Debug, Clone)]
pub struct CpInfoRefs {
    pub tag: &'static str,
    pub class_index: u16,
    pub name_and_type_index: u16,
}

#[derive(Debug, Clone)]
pub struct CpInfoNameAndType {
    pub tag: &'static str,
    pub name_index: u16,
    pub descriptor_index: u16,
}

#[derive(Debug, Clone)]
pub struct CpInfoUtf8 {
    pub tag: &'static str,
    pub data: String,
}
#[derive(Debug, Clone)]
pub struct CpInfoString {
    pub tag: &'static str,
    pub string_index: u16,
}

#[derive(Debug, Clone)]
pub struct CpInfoInvokeDynamic {
    pub tag: &'static str,
    pub bootstrap_method_attr_index: u16,
    pub name_and_type_index: u16,
}

#[derive(Debug, Clone)]
pub struct CpInfoMethodHandle {
    pub tag: &'static str,
    pub reference_kind: u8,
    pub reference_index: u16,
}

#[derive(Debug, Clone)]
pub struct CpInfoMethodType {
    pub tag: &'static str,
    pub descriptor_index: u16,
}
