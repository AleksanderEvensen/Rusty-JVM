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
