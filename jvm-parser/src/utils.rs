pub fn read_u4(bytes: &mut Vec<u8>) -> u32 {
    let mut read_bytes: [u8; 4] = Default::default();

    bytes.drain(0..4).enumerate().for_each(|(i, byte)| {
        read_bytes[i] = byte;
    });
    u32::from_be_bytes(read_bytes)
}

pub fn read_bytes(bytes: &mut Vec<u8>, length: usize) -> Vec<u8> {
    bytes.drain(0..length).collect()
}

pub fn read_u2(bytes: &mut Vec<u8>) -> u16 {
    let mut read_bytes: [u8; 2] = Default::default();

    bytes.drain(0..2).enumerate().for_each(|(i, byte)| {
        read_bytes[i] = byte;
    });
    u16::from_be_bytes(read_bytes)
}

pub fn read_u1(bytes: &mut Vec<u8>) -> u8 {
    bytes.remove(0)
}
