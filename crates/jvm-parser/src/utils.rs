pub mod big_endian {
    pub fn read_u4(bytes: &mut Vec<u8>) -> u32 {
        u32::from_be_bytes([
            bytes.remove(0),
            bytes.remove(0),
            bytes.remove(0),
            bytes.remove(0),
        ])
    }

    pub fn read_bytes(bytes: &mut Vec<u8>, length: usize) -> Vec<u8> {
        bytes.drain(0..length).collect()
    }

    pub fn read_u2(bytes: &mut Vec<u8>) -> u16 {
        u16::from_be_bytes([bytes.remove(0), bytes.remove(0)])
    }
    pub fn read_u1(bytes: &mut Vec<u8>) -> u8 {
        u8::from_be_bytes([bytes.remove(0)])
    }
}

pub mod little_endian {
    pub fn read_u4(bytes: &mut Vec<u8>) -> u32 {
        u32::from_le_bytes([
            bytes.remove(0),
            bytes.remove(0),
            bytes.remove(0),
            bytes.remove(0),
        ])
    }

    pub fn read_bytes(bytes: &mut Vec<u8>, length: usize) -> Vec<u8> {
        bytes.drain(0..length).collect()
    }

    pub fn read_u2(bytes: &mut Vec<u8>) -> u16 {
        u16::from_le_bytes([bytes.remove(0), bytes.remove(0)])
    }
    pub fn read_u1(bytes: &mut Vec<u8>) -> u8 {
        u8::from_le_bytes([bytes.remove(0)])
    }
}
