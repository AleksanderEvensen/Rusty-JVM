use std::collections::HashMap;

use nom::{IResult, multi::{length_data, fold_many_m_n}, combinator::map, number::complete::{be_i32, be_f32, be_i64, be_f64, be_u16, be_u8}, sequence::tuple};

#[derive(Debug, Clone)]
pub struct ConstantPool {
    count: u16,
    entries: HashMap<u16, CpInfo>,
}

impl ConstantPool {

	pub fn get_utf8<I>(&self, idx: I) -> Option<&String> where I: Into<u16> {
		let CpInfo::Utf8 { text } = self.entries.get(&idx.into())? else {
			return None;
		};
		return Some(text);
	}


}

#[derive(Debug, Clone)]
pub enum CpInfo {
    Utf8 {
        text: String,
    },
    Integer {
        value: i32,
    },

    Float {
        value: f32,
    },

    Long {
        value: i64,
    },

    Double {
        value: f64,
    },

    Class {
        name_index: u16,
    },
    String {
        string_index: u16,
    },
    Ref {
        class_index: u16,
        name_and_type: u16,
    },

    NameAndType {
        name_index: u16,
        descriptor_index: u16,
    },

    MethodHandle {
        reference_kind: u8,
        reference_index: u16,
    },

    MethodType {
        descriptor_index: u16,
    },

    Dynamic {
        bootstrap_method_attr_index: u16,
        name_and_type_index: u16,
    },

    Module {
        name_index: u16,
    },

    Package {
        name_index: u16,
    },
}

enum CpTags {
    Utf8,
    Integer,
    Float,
    Long,
    Double,
    Class,
    String,
    Ref,
    NameAndType,
    MethodHandle,
    MethodType,
    Dynamic,
    Module,
    Package,
}

impl From<u8> for CpTags {
    fn from(value: u8) -> Self {
        match value {
            1 => CpTags::Utf8,
            3 => CpTags::Integer,
            4 => CpTags::Float,
            5 => CpTags::Long,
            6 => CpTags::Double,
            7 => CpTags::Class,
            8 => CpTags::String,
            9 | 10 | 11 => CpTags::Ref,
            12 => CpTags::NameAndType,
            15 => CpTags::MethodHandle,
            16 => CpTags::MethodType,
            17 | 18 => CpTags::Dynamic,
            19 => CpTags::Module,
            20 => CpTags::Package,

            tag_value => unreachable!(
                "For some reason a ConstantPool tag value isn't implemented tag value: {tag_value}"
            ),
        }
    }
}

impl CpTags {
    fn parse_tag<'a>(tag: CpTags, bytes: &[u8]) -> IResult<&[u8], CpInfo> {
        match tag {
            CpTags::Utf8 => map(length_data(be_u16), |v| CpInfo::Utf8 {
                text: String::from_utf8_lossy(v).to_string(),
            })(bytes),
            CpTags::Integer => map(be_i32, |value| CpInfo::Integer { value })(bytes),
            CpTags::Float => map(be_f32, |value| CpInfo::Float { value })(bytes),
            CpTags::Long => map(be_i64, |value| CpInfo::Long { value })(bytes),
            CpTags::Double => map(be_f64, |value| CpInfo::Double { value })(bytes),
            CpTags::Class => map(be_u16, |name_index| CpInfo::Class { name_index })(bytes),
            CpTags::String => map(be_u16, |string_index| CpInfo::String { string_index })(bytes),
            CpTags::Ref => map(tuple((be_u16, be_u16)), |(class_index, name_and_type)| {
                CpInfo::Ref {
                    class_index,
                    name_and_type,
                }
            })(bytes),
            CpTags::NameAndType => {
                map(tuple((be_u16, be_u16)), |(name_index, descriptor_index)| {
                    CpInfo::NameAndType {
                        name_index,
                        descriptor_index,
                    }
                })(bytes)
            }
            CpTags::MethodHandle => map(
                tuple((be_u8, be_u16)),
                |(reference_kind, reference_index)| CpInfo::MethodHandle {
                    reference_kind,
                    reference_index,
                },
            )(bytes),
            CpTags::MethodType => map(be_u16, |descriptor_index| CpInfo::MethodType {
                descriptor_index,
            })(bytes),
            CpTags::Dynamic => map(
                tuple((be_u16, be_u16)),
                |(bootstrap_method_attr_index, name_and_type_index)| CpInfo::Dynamic {
                    bootstrap_method_attr_index,
                    name_and_type_index,
                },
            )(bytes),
            CpTags::Module => map(be_u16, |name_index| CpInfo::Module { name_index })(bytes),
            CpTags::Package => map(be_u16, |name_index| CpInfo::Package { name_index })(bytes),
        }
    }
}



pub(crate) fn parse_constant_pool(bytes: &[u8]) -> IResult<&[u8], ConstantPool> {
    let (bytes, pool_count) = be_u16(bytes)?;
    Ok(fold_many_m_n(
        0,
        pool_count as usize - 1,
        parse_constant_pool_value,
        HashMap::new,
        |mut acc: HashMap<_, _>, item| {
            let max_key = acc.keys().max();
            if let Some(max_key) = max_key {
                if let Some(max_entry) = acc.get(max_key) {
					let inc = match max_entry {
						CpInfo::Long { value: _ } | CpInfo::Double { value:_ } => 2,
						_ => 1,
					};

					// println!("#{} | {:?}", max_key + inc, item);
					
					acc.insert(max_key + inc, item);
                } else {
					unreachable!("The max index '{max_key}' should technically not be None, but for some god damn reason is ")
				}
            } else {
				// println!("#1 | {:?}", item);
                acc.insert(1, item);
            };
            acc
        },
    )(bytes)
    .map(|(bytes, entries)| {
        (
            bytes,
            ConstantPool {
                count: pool_count,
                entries,
            },
        )
    })?)
}

pub(crate) fn parse_constant_pool_value(bytes: &[u8]) -> IResult<&[u8], CpInfo> {
    let (bytes, tag) = map(be_u8, CpTags::from)(bytes)?;
    Ok(CpTags::parse_tag(tag, bytes)?)
}