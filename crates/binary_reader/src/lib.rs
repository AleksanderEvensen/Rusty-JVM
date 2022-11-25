use std::{
    fs::File,
    io::{Error, ErrorKind, Read},
};

macro_rules! read_fn {
    ($name:ident, $ty:ident, $bytes:expr) => {
        pub fn $name(&mut self) -> std::io::Result<$ty> {
            let endian = self.endian.clone();
            let data = self.read_bytes($bytes)?;
            match endian {
                Endian::Little => Ok($ty::from_le_bytes(data[..$bytes].try_into().unwrap())),
                Endian::Big => Ok($ty::from_be_bytes(data[..$bytes].try_into().unwrap())),
            }
        }
    };
}

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
}

impl BinaryReader {
    pub fn from_vec(vec: &Vec<u8>) -> Self {
        Self {
            data: vec.to_vec(),
            offset: 0,
            length: vec.len(),
            endian: Endian::Little,
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
    pub fn get_current_offset(&self) -> usize {
        self.offset
    }
    pub fn read_bytes(&mut self, bytes: usize) -> std::io::Result<&[u8]> {
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
        Ok(data)
    }
    pub fn peak_bytes(&self, bytes: usize) -> std::io::Result<&[u8]> {
        let data = self
            .data
            .get(self.offset..self.offset + bytes)
            .ok_or_else(|| {
                Error::new(
                    ErrorKind::UnexpectedEof,
                    format!("failed to read {} bytes from offset {}", bytes, self.offset),
                )
            })?;
        Ok(data)
    }

    pub fn find_next(&self, sequence: Vec<u8>) -> std::io::Result<usize> {
        self.find_from(sequence, self.offset.clone())
    }
    pub fn find(&self, sequence: Vec<u8>) -> std::io::Result<usize> {
        self.find_from(sequence, 0)
    }

    pub fn find_from(&self, sequence: Vec<u8>, mut offset: usize) -> std::io::Result<usize> {
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

    read_fn!(read_u8, u8, 1);
    read_fn!(read_u16, u16, 2);
    read_fn!(read_u32, u32, 4);
    read_fn!(read_u64, u64, 8);

    read_fn!(read_i8, i8, 1);
    read_fn!(read_i16, i16, 2);
    read_fn!(read_i32, i32, 4);
    read_fn!(read_i64, i64, 8);
}
