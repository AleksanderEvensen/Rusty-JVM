use std::{
    fs::File,
    io::{Error, ErrorKind, Read},
};

// use anyhow::{Context, Result};

#[derive(Clone, Copy)]
pub enum Endian {
    Little,
    Big,
}

pub struct BinaryReader {
    /// The buffer
    data: Vec<u8>,
    /// The current offset (position) in the buffer
    offset: usize,
    /// The buffer length
    length: usize,
    /// Which endian to use when reading
    endian: Endian,
    /// If the push_index function has been ran, the Option contains a value at that position, and can be returned to this position later using the pop_index function
    push_offset: Option<usize>,
}

impl BinaryReader {
    pub fn from_vec(vec: &Vec<u8>) -> Self {
        Self {
            data: vec.to_vec(),
            offset: 0,
            length: vec.len(),
            endian: Endian::Little,
            push_offset: None,
        }
    }

    pub fn from_file(file: &mut File) -> std::io::Result<Self> {
        let mut data = vec![];
        let length = file.read_to_end(&mut data).unwrap();
        Ok(Self {
            data: data,
            length: length,
            offset: 0,
            endian: Endian::Little,
            push_offset: None,
        })
    }
}

impl BinaryReader {
    pub fn set_endian<'a>(&'a mut self, endian: Endian) -> &'a mut Self {
        self.endian = endian;
        self
    }
    pub fn move_to<'a>(&'a mut self, offset: usize) -> &'a mut Self {
        self.offset = offset;
        self
    }
    pub fn jump<'a>(&'a mut self, jump_by: usize) -> &'a mut Self {
        self.offset += jump_by;
        self
    }

    pub fn push_index<'a>(&'a mut self) -> &'a mut Self {
        self.push_offset = Some(self.offset);
        self
    }
    pub fn pop_index<'a>(&'a mut self) -> &'a mut Self {
        if let Some(offset) = self.push_offset {
            self.offset = offset;
            self.push_offset = None;
        }
        self
    }

    pub fn get_current_offset(&self) -> usize {
        self.offset
    }
    pub fn read_bytes(&mut self, bytes: usize) -> std::io::Result<Vec<u8>> {
        let data = self
            .data
            .get(self.offset..self.offset + bytes)
            .ok_or_else(|| {
                Error::new(
                    ErrorKind::UnexpectedEof,
                    format!("failed to read {} bytes from offset {}", bytes, self.offset),
                )
            })?;
        self.offset += bytes;
        Ok(data.to_vec())
    }
    pub fn peak_bytes(&self, bytes: usize) -> std::io::Result<Vec<u8>> {
        let data = self
            .data
            .get(self.offset..self.offset + bytes)
            .ok_or_else(|| {
                Error::new(
                    ErrorKind::UnexpectedEof,
                    format!("failed to read {} bytes from offset {}", bytes, self.offset),
                )
            })?;
        Ok(data.to_vec())
    }

    pub fn find_next(&self, sequence: &Vec<u8>) -> std::io::Result<usize> {
        self.find_from(sequence, self.offset.clone())
    }
    pub fn find(&self, sequence: &Vec<u8>) -> std::io::Result<usize> {
        self.find_from(sequence, 0)
    }

    pub fn find_from(&self, sequence: &Vec<u8>, mut offset: usize) -> std::io::Result<usize> {
        let bytes = sequence.len();

        while offset + bytes < self.length {
            let data = self.data.get(offset..offset + bytes).ok_or_else(|| {
                Error::new(
                    ErrorKind::UnexpectedEof,
                    format!("failed to read {} bytes from offset {}", bytes, offset),
                )
            })?;

            if sequence[0] == data[0] {
                let mut is_equal = true;
                for (i, v) in sequence.iter().enumerate() {
                    if data[i] != *v {
                        is_equal = false;
                    }
                }
                if is_equal {
                    return Ok(offset);
                }
            }

            offset += 1;
        }

        Err(Error::new(
            ErrorKind::NotFound,
            format!(
                "Could not find the sequence {:?} after the offset {:X?}",
                sequence, self.offset
            ),
        ))
    }

    pub fn read_string_u16_length(&mut self) -> std::io::Result<String> {
        let length = *self.read::<u16>().unwrap();
        self.read_string(length as usize)
    }

    pub fn read_string(&mut self, length: usize) -> std::io::Result<String> {
        Ok(
            String::from_utf8(self.read_bytes(length)?.to_vec()).map_err(|err| {
                Error::new(
                    ErrorKind::InvalidData,
                    format!("Failed to parse Utf8 Bytes to Utf8String: {:?}", err),
                )
            })?,
        )
    }

    pub fn read<T: FromBinaryReader>(&mut self) -> std::io::Result<Box<T>> {
        T::from_byte_reader(self)
    }
}

pub trait FromBinaryReader {
    fn from_byte_reader(reader: &mut BinaryReader) -> std::io::Result<Box<Self>>;
}

macro_rules! impl_from_binary_reader {
    ($ty:ident, $bytes:expr) => {
        impl FromBinaryReader for $ty {
            fn from_byte_reader(reader: &mut BinaryReader) -> std::io::Result<Box<$ty>> {
                let endian = reader.endian.clone();
                let data = reader.read_bytes($bytes)?;
                match endian {
                    Endian::Little => Ok(Box::new($ty::from_le_bytes(
                        data[..$bytes].try_into().unwrap(),
                    ))),
                    Endian::Big => Ok(Box::new($ty::from_be_bytes(
                        data[..$bytes].try_into().unwrap(),
                    ))),
                }
            }
        }
    };
}

impl_from_binary_reader!(u8, 1);
impl_from_binary_reader!(i8, 1);
impl_from_binary_reader!(u16, 2);
impl_from_binary_reader!(i16, 2);
impl_from_binary_reader!(u32, 4);
impl_from_binary_reader!(i32, 4);
impl_from_binary_reader!(u64, 8);
impl_from_binary_reader!(i64, 8);

impl_from_binary_reader!(f32, 4);
impl_from_binary_reader!(f64, 8);
