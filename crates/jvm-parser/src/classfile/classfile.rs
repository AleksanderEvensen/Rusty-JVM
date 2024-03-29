use crate::classfile::attributes::{
    AttributeInfo, AttributeInfoData, BootstrapMethod, BootstrapMethodsAttribute, CodeAttribute,
    ExceptionTable, LineNumber, LineNumberTableAttribute, SourceFileAttribute,
};
use crate::classfile::constant_pool::ConstantPool;
use byte_reader::{ByteReader, Endian};
use std::{error::Error, path::PathBuf};

use super::attributes::{
    ConstantValueAttribute, EnclosingMethodAttribute, ExceptionsAttribute,
    LocalVariableTableAttribute, LocalVariableTableEntry, LocalVariableTypeTableAttribute,
    SignatureAttribute, StackMapTableAttribute,
};

type AccessFlags = u16;

#[allow(non_snake_case)]
pub mod ClassAccessFlags {
    pub const ACC_PUBLIC: u16 = 0x001;
    pub const ACC_FINAL: u16 = 0x0010;
    pub const ACC_SUPER: u16 = 0x0020;
    pub const ACC_INTERFACE: u16 = 0x0200;
    pub const ACC_ABSTRACT: u16 = 0x0040;
    pub const ACC_SYNTHETIC: u16 = 0x1000;
    pub const ACC_ANNOTATION: u16 = 0x2000;
    pub const ACC_ENUM: u16 = 0x4000;
}

#[allow(non_snake_case)]
pub mod MethodAccessFlags {
    pub const ACC_PUBLIC: u16 = 0x0001;
    pub const ACC_PRIVATE: u16 = 0x0002;
    pub const ACC_PROTECTED: u16 = 0x0004;
    pub const ACC_STATIC: u16 = 0x0008;
    pub const ACC_FINAL: u16 = 0x0010;
    pub const ACC_SYNCHRONIZED: u16 = 0x0020;
    pub const ACC_BRIDGE: u16 = 0x0040;
    pub const ACC_VARARGS: u16 = 0x0080;
    pub const ACC_NATIVE: u16 = 0x0100;
    pub const ACC_ABSTRACT: u16 = 0x0400;
    pub const ACC_STRICT: u16 = 0x0800;
    pub const ACC_SYNTHETIC: u16 = 0x1000;
}

#[allow(non_snake_case)]
pub mod FieldAccessFlags {
    pub const ACC_PUBLIC: u16 = 0x0001;
    pub const ACC_PRIVATE: u16 = 0x0002;
    pub const ACC_PROTECTED: u16 = 0x0004;
    pub const ACC_STATIC: u16 = 0x0008;
    pub const ACC_FINAL: u16 = 0x0010;
    pub const ACC_VOLATILE: u16 = 0x0040;
    pub const ACC_TRANSIENT: u16 = 0x0080;
    pub const ACC_SYNTHETIC: u16 = 0x1000;
    pub const ACC_ENUM: u16 = 0x4000;
}
#[derive(Debug)]
pub struct JavaClass {
    pub magic: u32,
    pub minor_version: u16,
    pub major_version: u16,
    pub constant_pool: ConstantPool,
    pub access_flags: AccessFlags,
    pub this_class: u16,
    pub super_class: u16,
    pub interfaces: Vec<u16>,
    pub fields: Vec<FieldInfo>,
    pub methods: Vec<MethodInfo>,
    pub attributes: Vec<AttributeInfo>,
}

impl JavaClass {
    pub fn from_file(path: &PathBuf) -> Result<JavaClass, Box<dyn Error>> {
        JavaClass::from_bytes(&std::fs::read(path)?)
    }
    pub fn from_bytes(bytes: &Vec<u8>) -> Result<JavaClass, Box<dyn Error>> {
        let mut reader = ByteReader::from_vec(bytes);
        reader.set_endian(Endian::Big);

        let mut class = JavaClass {
            magic: reader.read()?,
            minor_version: reader.read()?,
            major_version: reader.read()?,
            constant_pool: ConstantPool::from_reader(&mut reader)?,
            access_flags: reader.read()?,
            this_class: reader.read()?,
            super_class: reader.read()?,
            interfaces: (0..reader.read::<u16>()?)
                .map(|_| reader.read::<u16>().unwrap())
                .collect::<Vec<u16>>(),
            fields: vec![],
            methods: vec![],
            attributes: vec![],
        };

        class.fields = JavaClass::parse_fields(&mut reader, &class.constant_pool)?;

        class.methods = JavaClass::parse_methods(&mut reader, &class.constant_pool)?;

        class.attributes = JavaClass::parse_attributes(&mut reader, &class.constant_pool)?;

        Ok(class)
    }

    fn parse_methods(
        reader: &mut ByteReader,
        constant_pool: &ConstantPool,
    ) -> Result<Vec<MethodInfo>, Box<dyn Error>> {
        let method_count: u16 = reader.read()?;
        let mut methods = vec![];

        for _ in 0..method_count as usize {
            let mut method = MethodInfo {
                access_flags: reader.read()?,
                name_index: reader.read()?,
                descriptor_index: reader.read()?,
                attributes: vec![],
            };
            method.attributes = JavaClass::parse_attributes(reader, constant_pool)?;
            methods.push(method);
        }

        Ok(methods)
    }

    fn parse_attributes(
        reader: &mut ByteReader,
        constant_pool: &ConstantPool,
    ) -> Result<Vec<AttributeInfo>, Box<dyn Error>> {
        let attribute_count: u16 = reader.read()?;
        let mut attributes = vec![];

        #[allow(unused_variables)]
        for i in 0..attribute_count.to_owned() as usize {
            let attribute_name_index = reader.read()?;
            let attribute_length = reader.read::<u32>()?;

            let attribute_tag = constant_pool.get_utf8_at(attribute_name_index).unwrap();

            let attribute = match attribute_tag.data.as_str() {
                "Code" => {
                    let max_stack = reader.read()?;
                    let max_locals = reader.read()?;
                    let code_length: u32 = reader.read()?;
                    let code = reader.read_bytes(code_length as usize)?;
                    let exception_table_length: u16 = reader.read()?;

                    let mut exception_table = vec![];

                    for _ in 0..exception_table_length as usize {
                        exception_table.push(ExceptionTable {
                            start_pc: reader.read()?,
                            end_pc: reader.read()?,
                            handler_pc: reader.read()?,
                            catch_type: reader.read()?,
                        });
                    }

                    let attribute_info = JavaClass::parse_attributes(reader, constant_pool)?;

                    AttributeInfoData::Code(CodeAttribute {
                        max_stack,
                        max_locals,
                        code,
                        exception_table,
                        attribute_info,
                    })
                }

                "LineNumberTable" => {
                    let line_number_table_length: u16 = reader.read()?;
                    let mut line_number_table = vec![];

                    for _ in 0..line_number_table_length as usize {
                        line_number_table.push(LineNumber {
                            start_pc: reader.read()?,
                            line_number: reader.read()?,
                        })
                    }

                    AttributeInfoData::LineNumberTable(LineNumberTableAttribute {
                        line_number_table,
                    })
                }

                "SourceFile" => AttributeInfoData::SourceFile(SourceFileAttribute {
                    sourcefile_index: reader.read()?,
                }),

                "BootstrapMethods" => {
                    let num_bootstrap_methods: u16 = reader.read()?;
                    let mut bootstrap_methods = vec![];

                    for _ in 0..num_bootstrap_methods as usize {
                        let bootstrap_method_ref = reader.read()?;
                        let num_bootstrap_arguments: u16 = reader.read()?;
                        let mut bootstrap_arguments = vec![];

                        for _ in 0..num_bootstrap_arguments as usize {
                            let arg_index = reader.read()?;
                            bootstrap_arguments.push(arg_index);
                        }

                        bootstrap_methods.push(BootstrapMethod {
                            bootstrap_method_ref,
                            bootstrap_arguments,
                        });
                    }

                    AttributeInfoData::BootstrapMethods(BootstrapMethodsAttribute {
                        attribute_name_index,
                        bootstrap_methods,
                    })
                }

                "Signature" => AttributeInfoData::Signature(SignatureAttribute {
                    signature_index: reader.read().unwrap(),
                }),

                "EnclosingMethod" => AttributeInfoData::EnclosingMethod(EnclosingMethodAttribute {
                    class_index: reader.read().unwrap(),
                    method_index: reader.read().unwrap(),
                }),

                "InnerClasses" => {
                    reader.jump(attribute_length as usize);
                    AttributeInfoData::None
                }

                "Exceptions" => AttributeInfoData::Exceptions(ExceptionsAttribute {
                    exception_index_table: (0..reader.read::<u16>().unwrap())
                        .map(|_| reader.read().unwrap())
                        .collect(),
                }),

                "StackMapTable" => {
                    reader.jump(attribute_length as usize);
                    // println!("StackMapTable not implemented");

                    AttributeInfoData::StackMapTable(StackMapTableAttribute { entries: vec![] })
                    /*
                    AttributeInfoData::StackMapTable(StackMapTableAttribute {
                        entries: (0..*reader.read::<u16>().unwrap())
                            .map(|_| {
                                let frame_type: u8 = *reader.read().unwrap();

                                match frame_type {
                                    0..=63 => StackMapFrame::SameFrame,
                                    64..=127 => StackMapFrame::SameLocalsStackItemFrame,
                                    247 => StackMapFrame::SameLocalsStackItemFrameExtended,
                                    248..=250 => StackMapFrame::ChopFrame,
                                    251 => StackMapFrame::SameFrameExtended,
                                    252..=254 => StackMapFrame::AppendFrame,
                                    255 => StackMapFrame::FullFrame,

                                    _ => unreachable!("This should be unreacable if everything"),
                                }
                            })
                            .collect(),
                    })
                    */
                }
                "ConstantValue" => AttributeInfoData::ConstantValue(ConstantValueAttribute {
                    constantvalue_index: reader.read().unwrap(),
                }),

                "LocalVariableTable" => {
                    AttributeInfoData::LocalVariableTable(LocalVariableTableAttribute {
                        local_variable_table: (0..reader.read::<u16>().unwrap())
                            .map(|_| LocalVariableTableEntry {
                                start_pc: reader.read().unwrap(),
                                length: reader.read().unwrap(),
                                name_index: reader.read().unwrap(),
                                signature_descriptor_index: reader.read().unwrap(),
                                index: reader.read().unwrap(),
                            })
                            .collect(),
                    })
                }

                "LocalVariableTypeTable" => {
                    AttributeInfoData::LocalVariableTypeTable(LocalVariableTypeTableAttribute {
                        local_variable_type_table: (0..reader.read::<u16>().unwrap())
                            .map(|_| LocalVariableTableEntry {
                                start_pc: reader.read().unwrap(),
                                length: reader.read().unwrap(),
                                name_index: reader.read().unwrap(),
                                signature_descriptor_index: reader.read().unwrap(),
                                index: reader.read().unwrap(),
                            })
                            .collect(),
                    })
                }

                "Synthetic" => AttributeInfoData::Synthetic,

                // Optional to implement
                "SourceDebugExtension"
                | "Deprecated"
                | "RuntimeVisibleAnnotations"
                | "RuntimeInvisibleAnnotations"
                | "RuntimeVisibleParameterAnnotations"
                | "RuntimeInvisibleParameterAnnotations"
                | "RuntimeVisibleTypeAnnotations"
                | "RuntimeInvisibleTypeAnnotations"
                | "AnnotationDefault"
                | "MethodParameters"
                | "Module"
                | "ModulePackages"
                | "ModuleMainClass" => {
                    // Just skip over them
                    reader.jump(attribute_length as usize);
                    // Add it with the attribute tag (so we can see it is skipped)
                    AttributeInfoData::NoneAnnotated(attribute_tag.data.clone())
                }

                not_implemented_type => {
                    // reader.jump(attribute_length as usize);

                    // println!(
                    //     "#{i} = Skipping parsing for attribute: {}",
                    //     not_implemented_type
                    // );

                    // AttributeInfoData::None
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

        Ok(attributes)
    }

    fn parse_fields(
        reader: &mut ByteReader,
        constant_pool: &ConstantPool,
    ) -> Result<Vec<FieldInfo>, Box<dyn Error>> {
        let field_count: u16 = reader.read()?;

        let mut fields = vec![];
        for _ in 0..field_count {
            fields.push(FieldInfo {
                access_flags: reader.read()?,
                name_index: reader.read()?,
                descriptor_index: reader.read()?,
                attributes: JavaClass::parse_attributes(reader, constant_pool)?,
            })
        }

        Ok(fields)
    }
}

impl JavaClass {
    pub fn get_method_by_name(&self, name: &String) -> Option<&MethodInfo> {
        self.methods.iter().find(|method| {
            &self
                .constant_pool
                .get_utf8_at(method.name_index)
                .unwrap()
                .data
                == name
        })
    }
}

#[derive(Debug)]
pub struct MethodInfo {
    pub access_flags: AccessFlags,
    pub name_index: u16,
    pub descriptor_index: u16,
    pub attributes: Vec<AttributeInfo>,
}

#[derive(Debug)]
pub struct FieldInfo {
    pub access_flags: AccessFlags,
    pub name_index: u16,
    pub descriptor_index: u16,
    pub attributes: Vec<AttributeInfo>,
}
