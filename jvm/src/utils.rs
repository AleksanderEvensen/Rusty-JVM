use std::str::Chars;

#[derive(Debug, PartialEq, Eq)]
pub enum DescriptorTypes {
    Void,
    Byte,
    Char,
    Double,
    Float,
    Int,
    Long,
    Short,
    Boolean,
    Class(String),
    Array(String, u16),
}

#[derive(Debug, PartialEq, Eq)]
pub struct DescriptorResult {
    pub return_value: DescriptorTypes,
    pub parameters: Vec<DescriptorTypes>,
}

pub fn parse_descriptor(descriptor: &String) -> DescriptorResult {
    let mut chars = descriptor.chars().into_iter();

    let mut parameters = vec![];
    let mut return_value = DescriptorTypes::Void;

    let mut inside_parens = false;

    while let Some(value) = chars.next() {
        let base_type = match value {
            'B' => DescriptorTypes::Byte,
            'C' => DescriptorTypes::Char,
            'D' => DescriptorTypes::Boolean,
            'F' => DescriptorTypes::Float,
            'I' => DescriptorTypes::Int,
            'J' => DescriptorTypes::Long,
            'S' => DescriptorTypes::Short,
            'Z' => DescriptorTypes::Boolean,
            'L' => DescriptorTypes::Class("".to_string()),
            '[' => DescriptorTypes::Array("".to_string(), 0),
            'V' => DescriptorTypes::Void,
            _ => DescriptorTypes::Void,
        };

        match value {
            '(' => inside_parens = true,
            ')' => inside_parens = false,

            _ => match inside_parens {
                true => match base_type {
                    DescriptorTypes::Class(_) => parameters.push(parse_class(&mut chars)),
                    DescriptorTypes::Array(_, _) => parameters.push(parse_array(&mut chars)),
                    v => parameters.push(v),
                },

                false => match base_type {
                    DescriptorTypes::Class(_) => return_value = parse_class(&mut chars),
                    DescriptorTypes::Array(_, _) => return_value = parse_array(&mut chars),
                    v => return_value = v,
                },
            },
        }
    }

    DescriptorResult {
        return_value,
        parameters,
    }
}

fn parse_array(iter: &mut Chars) -> DescriptorTypes {
    let mut array_dim = 1;
    let mut array_type = String::new();

    while let Some(value) = iter.next() {
        match value {
            '[' => array_dim += 1,
            'L' => {
                if let DescriptorTypes::Class(class_type) = parse_class(iter) {
                    array_type = class_type;
                    break;
                }
            }
            v => {
                array_type.push(v);
                break;
            }
        }
    }

    DescriptorTypes::Array(array_type, array_dim)
}

fn parse_class(iter: &mut Chars) -> DescriptorTypes {
    let mut class_type = String::new();

    while let Some(value) = iter.next() {
        if value == ';' {
            break;
        }
        class_type.push(value);
    }

    DescriptorTypes::Class(class_type)
}

#[cfg(test)]
mod descriptor_tests {
    use super::{parse_descriptor, DescriptorResult, DescriptorTypes};

    #[test]
    fn parsing_byte_void() {
        let byte_void = parse_descriptor(&"(B)V".to_string());
        assert_eq!(
            byte_void,
            DescriptorResult {
                return_value: DescriptorTypes::Void,
                parameters: vec![DescriptorTypes::Byte]
            }
        )
    }
    #[test]
    fn parsing_char_bool() {
        let char_bool = parse_descriptor(&"(C)Z".to_string());
        assert_eq!(
            char_bool,
            DescriptorResult {
                return_value: DescriptorTypes::Boolean,
                parameters: vec![DescriptorTypes::Char]
            }
        )
    }
    #[test]
    fn parsing_class_bool_array() {
        let class_bool_array = parse_descriptor(&"(Ljava/io/PrintStream;)[B".to_string());

        assert_eq!(
            class_bool_array,
            DescriptorResult {
                return_value: DescriptorTypes::Array("B".to_string(), 1),
                parameters: vec![DescriptorTypes::Class("java/io/PrintStream".to_string())]
            }
        )
    }

    #[test]
    fn parsing_multidim_array() {
        let multidim_array = parse_descriptor(&"()[[[[B".to_string());
        assert_eq!(
            multidim_array,
            DescriptorResult {
                return_value: DescriptorTypes::Array("B".to_string(), 4),
                parameters: vec![]
            }
        )
    }
}
