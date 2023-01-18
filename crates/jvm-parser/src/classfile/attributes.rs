#[derive(Debug, Default, Clone)]
pub struct AttributeInfo {
    pub attribute_name_index: u16,
    pub attribute: AttributeInfoData,
}

#[derive(Debug, Default, Clone)]
pub enum AttributeInfoData {
    #[default]
    None,
    NoneAnnotated(String),

    Code(CodeAttribute),
    LineNumberTable(LineNumberTableAttribute),
    SourceFile(SourceFileAttribute),
    BootstrapMethods(BootstrapMethodsAttribute),
    LocalVariableTable(LocalVariableTableAttribute),
    LocalVariableTypeTable(LocalVariableTypeTableAttribute),
    Signature(SignatureAttribute),
    EnclosingMethod(EnclosingMethodAttribute),
    Exceptions(ExceptionsAttribute),
    StackMapTable(StackMapTableAttribute),
    ConstantValue(ConstantValueAttribute),
    Synthetic,
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

#[derive(Debug, Default, Clone)]
pub struct BootstrapMethodsAttribute {
    pub attribute_name_index: u16,
    pub bootstrap_methods: Vec<BootstrapMethod>,
}

#[derive(Debug, Default, Clone)]
pub struct BootstrapMethod {
    pub bootstrap_method_ref: u16,
    pub bootstrap_arguments: Vec<u16>,
}

#[derive(Debug, Default, Clone)]
pub struct SignatureAttribute {
    pub signature_index: u16,
}

#[derive(Debug, Default, Clone)]
pub struct LocalVariableTableAttribute {
    pub local_variable_table: Vec<LocalVariableTableEntry>,
}
#[derive(Debug, Default, Clone)]
pub struct LocalVariableTypeTableAttribute {
    pub local_variable_type_table: Vec<LocalVariableTableEntry>,
}
#[derive(Debug, Default, Clone)]
pub struct LocalVariableTableEntry {
    pub start_pc: u16,
    pub length: u16,
    pub name_index: u16,
    pub signature_descriptor_index: u16,
    pub index: u16,
}

#[derive(Debug, Default, Clone)]
pub struct EnclosingMethodAttribute {
    pub class_index: u16,
    pub method_index: u16,
}

#[derive(Debug, Default, Clone)]
pub struct ExceptionsAttribute {
    pub exception_index_table: Vec<u16>,
}

#[derive(Debug, Default, Clone)]
pub struct StackMapTableAttribute {
    pub entries: Vec<StackMapFrame>,
}

#[derive(Debug, Default, Clone)]
pub enum StackMapFrame {
    #[default]
    None,

    SameFrame,
    SameFrameExtended,
    SameLocalsStackItemFrame,
    SameLocalsStackItemFrameExtended,
    ChopFrame,
    AppendFrame,
    FullFrame,
}

#[derive(Debug, Default, Clone)]
pub struct ConstantValueAttribute {
    pub constantvalue_index: u16,
}
