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
    Array(Box<DescriptorTypes>, u16),
}

pub enum DescriptorValues {
    Void,
    Byte(u8),
    Char(char),
    Double(f64),
    Float(f32),
    Int(i32),
    Long(i64),
    Short(i16),
    Boolean(bool),
    Class(String),
    Array(Vec<DescriptorValues>, u16),
}

#[derive(Debug, PartialEq, Eq)]
pub struct Descriptor {
    pub return_value: DescriptorTypes,
    pub parameters: Vec<DescriptorTypes>,
}

pub fn parse_descriptor(descriptor: &String) -> Descriptor {
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
            'V' => DescriptorTypes::Void,
            'L' => DescriptorTypes::Class("".to_string()),
            '[' => DescriptorTypes::Array(Box::new(DescriptorTypes::Void), 0),
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

    Descriptor {
        return_value,
        parameters,
    }
}

fn parse_array(iter: &mut Chars) -> DescriptorTypes {
    let mut array_dim = 1;
    let mut array_type = Box::new(DescriptorTypes::Void);

    while let Some(value) = iter.next() {
        match value {
            '[' => array_dim += 1,
            'L' => {
                array_type = Box::new(parse_class(iter));
                break;
            }
            'B' => {
                array_type = Box::new(DescriptorTypes::Byte);
                break;
            }
            'C' => {
                array_type = Box::new(DescriptorTypes::Char);
                break;
            }
            'D' => {
                array_type = Box::new(DescriptorTypes::Boolean);
                break;
            }
            'F' => {
                array_type = Box::new(DescriptorTypes::Float);
                break;
            }
            'I' => {
                array_type = Box::new(DescriptorTypes::Int);
                break;
            }
            'J' => {
                array_type = Box::new(DescriptorTypes::Long);
                break;
            }
            'S' => {
                array_type = Box::new(DescriptorTypes::Short);
                break;
            }
            'Z' => {
                array_type = Box::new(DescriptorTypes::Boolean);
                break;
            }
            'V' => {
                array_type = Box::new(DescriptorTypes::Void);
                break;
            }
            _ => panic!("Un handled array type \"{}\"", value),
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
    use super::{parse_descriptor, Descriptor, DescriptorTypes};

    #[test]
    fn parsing_byte_void() {
        let byte_void = parse_descriptor(&"(B)V".to_string());
        assert_eq!(
            byte_void,
            Descriptor {
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
            Descriptor {
                return_value: DescriptorTypes::Boolean,
                parameters: vec![DescriptorTypes::Char]
            }
        )
    }
    #[test]
    fn parsing_class_bool_array() {
        let class_bool_array = parse_descriptor(&"(Ljava/io/PrintStream;)[Z".to_string());

        assert_eq!(
            class_bool_array,
            Descriptor {
                return_value: DescriptorTypes::Array(Box::new(DescriptorTypes::Boolean), 1),
                parameters: vec![DescriptorTypes::Class("java/io/PrintStream".to_string())]
            }
        )
    }

    #[test]
    fn parsing_multidim_array() {
        let multidim_array = parse_descriptor(&"()[[[[B".to_string());
        assert_eq!(
            multidim_array,
            Descriptor {
                return_value: DescriptorTypes::Array(Box::new(DescriptorTypes::Byte), 4),
                parameters: vec![]
            }
        )
    }
}
