#![allow(dead_code)]

pub mod attributes;
pub mod constant_pool;

mod util;

pub use attributes::Attribute;
pub use constant_pool::ConstantPool;
pub use constant_pool::CpInfo;

use attributes::parse_attributes;
use constant_pool::parse_constant_pool;
use nom::{
    bytes::complete::tag, combinator::map, multi::fold_many_m_n, number::complete::be_u16,
    sequence::tuple, IResult,
};

#[derive(Debug)]
#[allow(unused_variables)]
pub struct ClassFile {
    version: Version,
    constant_pool: ConstantPool,
    class_info: ClassInfo,
    interfaces: Vec<u16>,
    fields: Vec<Field>,
    methods: Vec<Method>,
    attributes: Vec<Attribute>,
}

#[derive(Debug)]
pub struct Version {
    major: u16,
    minor: u16,
}

#[derive(Debug)]
pub struct ClassInfo {
    access_flags: u16,
    this_class: u16,
    super_class: u16,
}

#[derive(Debug)]
pub struct Field {
    access_flags: u16,
    name_index: u16,
    descriptor_index: u16,
    attributes: Vec<Attribute>,
}

#[derive(Debug)]
pub struct Method {
    access_flags: u16,
    name_index: u16,
    descriptor_index: u16,
    attributes: Vec<Attribute>,
}

fn parse_version(bytes: &[u8]) -> IResult<&[u8], Version> {
    Ok(map(tuple((be_u16, be_u16)), |(minor, major)| Version {
        minor,
        major,
    })(bytes)?)
}

fn parse_class_info(bytes: &[u8]) -> IResult<&[u8], ClassInfo> {
    Ok(map(
        tuple((be_u16, be_u16, be_u16)),
        |(access_flags, this_class, super_class)| ClassInfo {
            access_flags,
            this_class,
            super_class,
        },
    )(bytes)?)
}

fn parse_interfaces(bytes: &[u8]) -> IResult<&[u8], Vec<u16>> {
    let (mut bytes, length) = be_u16(bytes)?;
    let mut vec: Vec<u16> = vec![];

    for _ in 0..length {
        let (bytes2, v) = be_u16(bytes)?;
        vec.push(v);
        bytes = bytes2;
    }
    Ok((bytes, vec))
}

fn parse_fields<'a>(bytes: &'a [u8], constant_pool: ConstantPool) -> IResult<&'a [u8], Vec<Field>> {
    let (bytes, field_count) = be_u16(bytes)?;

    Ok(fold_many_m_n(
        0,
        field_count as usize,
        map(
            tuple((be_u16, be_u16, be_u16, parse_attributes(constant_pool))),
            |(access_flags, name_index, descriptor_index, attributes)| Field {
                access_flags,
                name_index,
                descriptor_index,
                attributes,
            },
        ),
        Vec::new,
        |mut acc, item| {
            acc.push(item);
            acc
        },
    )(bytes)?)
}

fn parse_methods<'a>(
    bytes: &'a [u8],
    constant_pool: ConstantPool,
) -> IResult<&'a [u8], Vec<Method>> {
    let (bytes, method_count) = be_u16(bytes)?;

    Ok(fold_many_m_n(
        0,
        method_count as usize,
        map(
            tuple((be_u16, be_u16, be_u16, parse_attributes(constant_pool))),
            |(access_flags, name_index, descriptor_index, attributes)| Method {
                access_flags,
                name_index,
                descriptor_index,
                attributes,
            },
        ),
        Vec::new,
        |mut acc, item| {
            acc.push(item);
            acc
        },
    )(bytes)?)
}

pub fn parse(bytes: &[u8]) -> IResult<&[u8], ClassFile> {
    let (bytes, (_magic, version, constant_pool, class_info, interfaces)) = tuple((
        tag(b"\xCA\xFE\xBA\xBE"),
        parse_version,
        parse_constant_pool,
        parse_class_info,
        parse_interfaces,
    ))(bytes)?;

    let (bytes, fields) = parse_fields(bytes, constant_pool.clone())?;
    let (bytes, methods) = parse_methods(bytes, constant_pool.clone())?;
    let (bytes, attributes) = parse_attributes(constant_pool.clone())(bytes)?;

    Ok((
        bytes,
        ClassFile {
            version,
            constant_pool,
            class_info,
            interfaces,
            fields,
            methods,
            attributes,
        },
    ))
}
