use std::str::Chars;

#[derive(Debug)]
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

#[derive(Debug)]
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
