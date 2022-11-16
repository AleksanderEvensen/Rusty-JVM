use std::default;

use crate::utils::Descriptor;

#[derive(Default, Debug, Clone)]
pub struct JVMClass {
    pub metadata: ClassMetadata,
    pub constant_pool: Vec<JVMCpInfo>,
    pub access_flags: Vec<ClassAccessFlags>,
    pub class_name: String,
    pub super_class_name: String,
    // pub interfaces: Vec<JVMInterfaces>, // TODO: Add when implementing interfaces
    // pub fields: Vec<JVMFieldInfo> // TODO: Add when implementing Fields
    pub methods: Vec<JVMMethod>,
}

#[derive(Default, Debug, Clone)]
pub enum JVMCpInfo {
    #[default]
    V,
}

#[derive(Default, Debug, Clone)]
pub struct ClassMetadata {
    pub magic: String,
    pub major_version: u16,
    pub minor_version: u16,
}

#[derive(Default, Debug, Clone)]
pub enum ClassAccessFlags {
    #[default]
    PUBLIC,
    FINAL,
    SUPER,
    INTERFACE,
    ABSTRACT,
    SYNTHETIC,
    ANNOTATION,
    ENUM,
}

#[derive(Debug)]
pub struct JVMMethod {
    acces_flags: Vec<MethodAccessFlags>,
    name: String,
    descriptor: Descriptor,
    attributes: Vec<JVMAttributes>,
}

#[derive(Default, Debug, Clone)]
pub enum MethodAccessFlags {
    #[default]
    PUBLIC,
    PRIVATE,
    PROTECTED,
    STATIC,
    FINAL,
    SYNCHRONIZED,
    BRIDGE,
    VARARGS,
    NATIVE,
    ABSTRACT,
    STRICT,
    SYNTHETIC,
}

#[derive(Default, Debug, Clone)]
pub struct JVMAttributes {
    pub attribute_name: String,
    pub attribute: JVMAttributeData,
}

#[derive(Default, Debug, Clone)]
pub enum JVMAttributeData {
    #[default]
    None,

    Code(CodeAttribute),
    LineNumberTable(LineNumberTableAttribute),
    SourceFile(SourceFileAttribute),
    BootstrapMethods(BootstrapMethodsAttribute),
}

#[derive(Debug, Default, Clone)]
pub struct CodeAttribute {
    pub max_stack: u16,
    pub max_locals: u16,
    pub code: Vec<u8>,
    pub exception_table: Vec<ExceptionTable>,
    pub attribute_info: Vec<JVMAttributes>,
}

#[derive(Debug, Default, Clone)]
pub struct SourceFileAttribute {
    pub sourcefile_index: u16,
}

#[derive(Debug, Default, Clone)]
pub struct LineNumberTableAttribute {
    pub line_number_table: Vec<LineNumber>,
}

#[derive(Debug, Default, Clone)]
pub struct LineNumber {
    pub start_pc: u16,
    pub line_number: u16,
}

#[derive(Debug, Default, Clone)]
pub struct ExceptionTable {
    pub start_pc: u16,
    pub end_pc: u16,
    pub handler_pc: u16,
    pub catch_type: u16,
}

#[derive(Debug, Default, Clone)]
pub struct BootstrapMethodsAttribute {
    pub attribute_name_index: u16,
    pub bootstrap_methods: Vec<BootstrapMethod>,
}
