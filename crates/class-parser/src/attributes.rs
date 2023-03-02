use nom::{
    combinator::map,
    multi::length_data,
    number::complete::{be_u16, be_u32, be_u8},
    sequence::tuple,
    IResult,
};

use crate::{util::length_to_vec, ConstantPool};

#[derive(Debug)]
pub enum Attribute {
    ConstantValue,
    Code {
        max_stack: u16,
        max_locals: u16,
        code: Vec<u8>,
    },
    StackMapTable,
    Exceptions,
    InnerClasses,
    EnclosingMethod,
    Synthetic,
    Signature,
    SourceFile {
        sourcefile_index: u16,
    },
    SourceDebugExtension,
    LineNumberTable,
    LocalVariableTable,
    LocalVariableTypeTable,
    Deprecated,
    RuntimeVisibleAnnotations,
    RuntimeInvisibleAnnotations,
    RuntimeVisibleParameterAnnotations,
    RuntimeInvisibleParameterAnnotations,
    RuntimeVisibleTypeAnnotations,
    RuntimeInvisibleTypeAnnotations,
    AnnotationDefault,
    BootstrapMethods,
    MethodParameters,
    Module,
    ModulePackages,
    ModuleMainClass,
    NestHost,
    NestMembers,
    Record,
    PermittedSubclasses,
}

enum AttributeTag {
    ConstantValue,
    Code,
    StackMapTable,
    Exceptions,
    InnerClasses,
    EnclosingMethod,
    Synthetic,
    Signature,
    SourceFile,
    SourceDebugExtension,
    LineNumberTable,
    LocalVariableTable,
    LocalVariableTypeTable,
    Deprecated,
    RuntimeVisibleAnnotations,
    RuntimeInvisibleAnnotations,
    RuntimeVisibleParameterAnnotations,
    RuntimeInvisibleParameterAnnotations,
    RuntimeVisibleTypeAnnotations,
    RuntimeInvisibleTypeAnnotations,
    AnnotationDefault,
    BootstrapMethods,
    MethodParameters,
    Module,
    ModulePackages,
    ModuleMainClass,
    NestHost,
    NestMembers,
    Record,
    PermittedSubclasses,
}

impl From<String> for AttributeTag {
    fn from(value: String) -> Self {
        match value.as_str() {
            "ConstantValue" => AttributeTag::ConstantValue,
            "Code" => AttributeTag::Code,
            "StackMapTable" => AttributeTag::StackMapTable,
            "Exceptions" => AttributeTag::Exceptions,
            "InnerClasses" => AttributeTag::InnerClasses,
            "EnclosingMethod" => AttributeTag::EnclosingMethod,
            "Synthetic" => AttributeTag::Synthetic,
            "Signature" => AttributeTag::Signature,
            "SourceFile" => AttributeTag::SourceFile,
            "SourceDebugExtension" => AttributeTag::SourceDebugExtension,
            "LineNumberTable" => AttributeTag::LineNumberTable,
            "LocalVariableTable" => AttributeTag::LocalVariableTable,
            "LocalVariableTypeTable" => AttributeTag::LocalVariableTypeTable,
            "Deprecated" => AttributeTag::Deprecated,
            "RuntimeVisibleAnnotations" => AttributeTag::RuntimeVisibleAnnotations,
            "RuntimeInvisibleAnnotations" => AttributeTag::RuntimeInvisibleAnnotations,
            "RuntimeVisibleParameterAnnotations" => {
                AttributeTag::RuntimeVisibleParameterAnnotations
            }
            "RuntimeInvisibleParameterAnnotations" => {
                AttributeTag::RuntimeInvisibleParameterAnnotations
            }
            "RuntimeVisibleTypeAnnotations" => AttributeTag::RuntimeVisibleTypeAnnotations,
            "RuntimeInvisibleTypeAnnotations" => AttributeTag::RuntimeInvisibleTypeAnnotations,
            "AnnotationDefault" => AttributeTag::AnnotationDefault,
            "BootstrapMethods" => AttributeTag::BootstrapMethods,
            "MethodParameters" => AttributeTag::MethodParameters,
            "Module" => AttributeTag::Module,
            "ModulePackages" => AttributeTag::ModulePackages,
            "ModuleMainClass" => AttributeTag::ModuleMainClass,
            "NestHost" => AttributeTag::NestHost,
            "NestMembers" => AttributeTag::NestMembers,
            "Record" => AttributeTag::Record,
            "PermittedSubclasses" => AttributeTag::PermittedSubclasses,

            _ => unreachable!(),
        }
    }
}

pub(crate) fn parse_attributes<'a>(
    constant_pool: ConstantPool,
) -> impl FnMut(&'a [u8]) -> IResult<&'a [u8], Vec<Attribute>> {
    move |bytes| {
        let (mut bytes, attribute_count) = be_u16(bytes)?;

        let mut attributes: Vec<Attribute> = vec![];

        for _ in 0..attribute_count {
            let (bytes2, name_index) = be_u16(bytes)?;
            let (bytes2, attrib_bytes) = length_data(be_u32)(bytes2)?;

            if let Some(attrib_name) = constant_pool.get_utf8(name_index) {
                let tag = AttributeTag::from(attrib_name.clone());

                let (_, value) = match tag {
                    AttributeTag::ConstantValue => {
                        todo!("Implement Attribute parsing for 'ConstantValue'")
                    }
                    AttributeTag::Code => map(
                        tuple((be_u16, be_u16, length_to_vec(be_u32, be_u8))),
                        |(max_stack, max_locals, code)| Attribute::Code {
                            max_stack,
                            max_locals,
                            code,
                        },
                    )(attrib_bytes)?,
                    AttributeTag::StackMapTable => {
                        todo!("Implement Attribute parsing for 'StackMapTable'")
                    }
                    AttributeTag::Exceptions => {
                        todo!("Implement Attribute parsing for 'Exceptions'")
                    }
                    AttributeTag::InnerClasses => {
                        todo!("Implement Attribute parsing for 'InnerClasses'")
                    }
                    AttributeTag::EnclosingMethod => {
                        todo!("Implement Attribute parsing for 'EnclosingMethod'")
                    }
                    AttributeTag::Synthetic => {
                        todo!("Implement Attribute parsing for 'Synthetic'")
                    }
                    AttributeTag::Signature => {
                        todo!("Implement Attribute parsing for 'Signature'")
                    }
                    AttributeTag::SourceFile => map(be_u16, |sourcefile_index| {
                        Attribute::SourceFile { sourcefile_index }
                    })(attrib_bytes)?,
                    AttributeTag::SourceDebugExtension => {
                        todo!("Implement Attribute parsing for 'SourceDebugExtension'")
                    }
                    AttributeTag::LineNumberTable => {
                        todo!("Implement Attribute parsing for 'LineNumberTable'")
                    }
                    AttributeTag::LocalVariableTable => {
                        todo!("Implement Attribute parsing for 'LocalVariableTable'")
                    }
                    AttributeTag::LocalVariableTypeTable => {
                        todo!("Implement Attribute parsing for 'LocalVariableTypeTable'")
                    }
                    AttributeTag::Deprecated => {
                        todo!("Implement Attribute parsing for 'Deprecated'")
                    }
                    AttributeTag::RuntimeVisibleAnnotations => {
                        todo!("Implement Attribute parsing for 'RuntimeVisibleAnnotations'")
                    }
                    AttributeTag::RuntimeInvisibleAnnotations => {
                        todo!("Implement Attribute parsing for 'RuntimeInvisibleAnnotations'")
                    }
                    AttributeTag::RuntimeVisibleParameterAnnotations => todo!(
                        "Implement Attribute parsing for 'RuntimeVisibleParameterAnnotations'"
                    ),
                    AttributeTag::RuntimeInvisibleParameterAnnotations => todo!(
                        "Implement Attribute parsing for 'RuntimeInvisibleParameterAnnotations'"
                    ),
                    AttributeTag::RuntimeVisibleTypeAnnotations => {
                        todo!("Implement Attribute parsing for 'RuntimeVisibleTypeAnnotations'")
                    }
                    AttributeTag::RuntimeInvisibleTypeAnnotations => {
                        todo!("Implement Attribute parsing for 'RuntimeInvisibleTypeAnnotations'")
                    }
                    AttributeTag::AnnotationDefault => {
                        todo!("Implement Attribute parsing for 'AnnotationDefault'")
                    }
                    AttributeTag::BootstrapMethods => {
                        todo!("Implement Attribute parsing for 'BootstrapMethods'")
                    }
                    AttributeTag::MethodParameters => {
                        todo!("Implement Attribute parsing for 'MethodParameters'")
                    }
                    AttributeTag::Module => todo!("Implement Attribute parsing for 'Module'"),
                    AttributeTag::ModulePackages => {
                        todo!("Implement Attribute parsing for 'ModulePackages'")
                    }
                    AttributeTag::ModuleMainClass => {
                        todo!("Implement Attribute parsing for 'ModuleMainClass'")
                    }
                    AttributeTag::NestHost => {
                        todo!("Implement Attribute parsing for 'NestHost'")
                    }
                    AttributeTag::NestMembers => {
                        todo!("Implement Attribute parsing for 'NestMembers'")
                    }
                    AttributeTag::Record => todo!("Implement Attribute parsing for 'Record'"),
                    AttributeTag::PermittedSubclasses => {
                        todo!("Implement Attribute parsing for 'PermittedSubclasses'")
                    }
                };
                attributes.push(value);
            }

            bytes = bytes2;
            // println!(
            //     "Name Index: {name_index} | Attrib Bytes: {attrib_bytes:?} | Rest Length: {}",
            //     bytes.len()
            // );
        }

        Ok((bytes, attributes))
    }
}
