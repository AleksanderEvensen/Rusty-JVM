#[derive(Debug, Default, Clone)]
pub struct AttributeInfo {
    pub attribute_name_index: u16,
    pub attribute: AttributeInfoData,
}

#[derive(Debug, Default, Clone)]
pub enum AttributeInfoData {
    #[default]
    None,

    Code(CodeAttribute),

    LineNumberTable(LineNumberTableAttribute),

    SourceFile(SourceFileAttribute),
}

#[derive(Debug, Default, Clone)]
pub struct CodeAttribute {
    pub max_stack: u16,
    pub max_locals: u16,
    pub code: Vec<u8>,
    pub exception_table: Vec<ExceptionTable>,
    pub attribute_info: Vec<AttributeInfo>,
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
