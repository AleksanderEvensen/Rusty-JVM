use byte_reader::ByteReader;

#[derive(Debug)]
pub enum CmpConditions {
    Equal,
    NotEqual,

    LessThan,
    LessOrEqual,

    GreaterOrEqual,
    GreaterThan,
}

#[allow(non_camel_case_types)]
#[derive(Debug)]
pub enum OpCodes {
    nop,
    dup,
    getstatic(u16),
    ldc(u8),
    ldc2_w(u16),
    ldc_w(u16),
    invokevirtual(u16),
    invokespecial(u16),
    invokestatic(u16),
    bipush(u8),
    sipush(u16),

    new(u16),
    anewarray(u16),

    istore_(u8),
    iconst_(i32),
    iload_(u8),

    iaload,

    fstore_(u8),
    fconst_(f32),
    fload_(u8),

    astore_(u8),
    aconst_null,
    aload_(u8),

    checkcast(u16),

    putfield(u16),
    getfield(u16),

    if_icmp(CmpConditions, u16),

    goto(u16),

    ireturn,
    areturn,
    Return,
}

pub fn parse_opcodes(opcode_bytes: &Vec<u8>) -> std::io::Result<Vec<OpCodes>> {
    let mut reader = ByteReader::from_vec(opcode_bytes);
    let mut opcodes = vec![];

    while let Ok(opcode_byte) = reader.read::<u8>() {
        let opcode = match opcode_byte {
            0x00 => OpCodes::nop,
            0x59 => OpCodes::dup,
            0xb2 => OpCodes::getstatic(reader.read()?),
            0x12 => OpCodes::ldc(reader.read()?),
            0x13 => OpCodes::ldc_w(reader.read()?),
            0x14 => OpCodes::ldc2_w(reader.read()?),
            0xb6 => OpCodes::invokevirtual(reader.read()?),
            0xb7 => OpCodes::invokespecial(reader.read()?),
            0xb8 => OpCodes::invokestatic(reader.read()?),
            0x10 => OpCodes::bipush(reader.read()?),
            0x11 => OpCodes::sipush(reader.read()?),

            0xbb => OpCodes::new(reader.read()?),
            0xbd => OpCodes::anewarray(reader.read()?),

            0x36 => OpCodes::istore_(reader.read()?),
            0x3b => OpCodes::istore_(0),
            0x3c => OpCodes::istore_(1),
            0x3d => OpCodes::istore_(2),
            0x3e => OpCodes::istore_(3),

            0x2 => OpCodes::iconst_(-1),
            0x3 => OpCodes::iconst_(0),
            0x4 => OpCodes::iconst_(1),
            0x5 => OpCodes::iconst_(2),
            0x6 => OpCodes::iconst_(3),
            0x7 => OpCodes::iconst_(4),

            0x15 => OpCodes::iload_(reader.read()?),
            0x1a => OpCodes::iload_(0),
            0x1b => OpCodes::iload_(1),
            0x1c => OpCodes::iload_(2),
            0x1d => OpCodes::iload_(3),

            0x2e => OpCodes::iaload,

            0x38 => OpCodes::fstore_(reader.read()?),
            0x43 => OpCodes::fstore_(0),
            0x44 => OpCodes::fstore_(1),
            0x45 => OpCodes::fstore_(2),
            0x46 => OpCodes::fstore_(3),

            0xb => OpCodes::fconst_(0.0),
            0xc => OpCodes::fconst_(1.0),
            0xd => OpCodes::fconst_(2.0),

            0x17 => OpCodes::fload_(reader.read()?),
            0x22 => OpCodes::fload_(0),
            0x23 => OpCodes::fload_(1),
            0x24 => OpCodes::fload_(2),
            0x25 => OpCodes::fload_(3),

            0x19 => OpCodes::aload_(reader.read()?),
            0x2a => OpCodes::aload_(0),
            0x2b => OpCodes::aload_(1),
            0x2c => OpCodes::aload_(2),
            0x2d => OpCodes::aload_(3),

            0x1 => OpCodes::aconst_null,

            0x3a => OpCodes::astore_(reader.read()?),
            0x4b => OpCodes::astore_(0),
            0x4c => OpCodes::astore_(1),
            0x4d => OpCodes::astore_(2),
            0x4e => OpCodes::astore_(3),

            0xc0 => OpCodes::checkcast(reader.read()?),

            0xb5 => OpCodes::putfield(reader.read()?),
            0xb4 => OpCodes::getfield(reader.read()?),

            0x9f => OpCodes::if_icmp(CmpConditions::Equal, reader.read()?),
            0xa0 => OpCodes::if_icmp(CmpConditions::NotEqual, reader.read()?),
            0xa1 => OpCodes::if_icmp(CmpConditions::LessThan, reader.read()?),
            0xa4 => OpCodes::if_icmp(CmpConditions::LessOrEqual, reader.read()?),
            0xa3 => OpCodes::if_icmp(CmpConditions::GreaterThan, reader.read()?),
            0xa2 => OpCodes::if_icmp(CmpConditions::GreaterOrEqual, reader.read()?),

            0xa7 => OpCodes::goto(reader.read()?),

            0xac => OpCodes::ireturn,
            0xb0 => OpCodes::areturn,
            0xb1 => OpCodes::Return,

            unknown_opcode => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::Unsupported,
                    format!("Unknown opcode: {unknown_opcode}"),
                ));
            }
        };

        opcodes.push(opcode);
    }

    Ok(opcodes)
}
