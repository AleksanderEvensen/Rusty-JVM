use std::collections::HashMap;

use nom::{
    bytes::complete::tag,
    combinator::map,
    multi::{fold_many_m_n, length_data},
    number::complete::{be_f32, be_f64, be_i32, be_i64, be_u16, be_u8},
    sequence::tuple,
    IResult, Parser, InputLength, InputTake, ToUsize, error::ParseError,
};

#[derive(Debug)]
pub struct ClassFile {
    version: Version,
    constant_pool: ConstantPool,
	class_info: ClassInfo,
	interfaces: Vec<u16>
}

#[derive(Debug)]
pub struct ConstantPool {
    count: u16,
    entries: HashMap<u16, CpInfo>,
}

#[derive(Debug)]
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

#[derive(Debug)]
#[allow(unused)]
pub struct Version {
    major: u16,
    minor: u16,
}

#[derive(Debug)]
pub struct ClassInfo {
	access_flags: u16,
	this_class: u16,
	super_class: u16
}

fn parse_version(bytes: &[u8]) -> IResult<&[u8], Version> {
    Ok(map(tuple((be_u16, be_u16)), |(minor, major)| Version {
        minor,
        major,
    })(bytes)?)
}

fn parse_constant_pool(bytes: &[u8]) -> IResult<&[u8], ConstantPool> {
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

					println!("#{} | {:?}", max_key + inc, item);
					
					acc.insert(max_key + inc, item);
                } else {
					unreachable!("The max index '{max_key}' should technically not be None, but for some god damn reason is ")
				}
            } else {
				println!("#1 | {:?}", item);
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

fn parse_constant_pool_value(bytes: &[u8]) -> IResult<&[u8], CpInfo> {
    let (bytes, tag) = map(be_u8, CpTags::from)(bytes)?;
    Ok(CpTags::parse_tag(tag, bytes)?)
}

fn parse_class_info(bytes: &[u8]) -> IResult<&[u8], ClassInfo> {
	Ok(map(tuple((be_u16,be_u16,be_u16)), |(access_flags, this_class, super_class)| ClassInfo { access_flags, this_class, super_class })(bytes)?)
}


fn length_vec_of<I,O,N,E,F,G>(mut f: F, mut g: G) -> impl FnMut(I) -> IResult<I,Vec<O>,E> where
I: Clone + InputLength + InputTake,
N: ToUsize,
F: Parser<I, N, E>,
G: Parser<I, O, E>,
E: ParseError<I>, {
	move |i: I| {
		let (mut i, length) = f.parse(i)?;
		let mut vec: Vec<O> = vec![];

		for _ in 0..length.to_usize() {
			let (i2, v) = g.parse(i)?;
			vec.push(v);
			i = i2;
		}

		Ok((i, vec))



	} 
}

pub fn parse(bytes: &[u8]) -> IResult<&[u8], ClassFile> {
    Ok(map(
        tuple((tag(b"\xCA\xFE\xBA\xBE"), parse_version, parse_constant_pool, parse_class_info, length_vec_of(be_u16, be_u16))),
        |(_, version, constant_pool, class_info, interfaces)| ClassFile {
            version,
            constant_pool,
			class_info,
			interfaces,
        },
    )(bytes)?)
}
