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
    dup2,
    dup_x1,
    pop,
    getstatic(u16),
    ldc(u8),
    ldc2_w(u16),
    ldc_w(u16),
    invokevirtual(u16),
    invokespecial(u16),
    invokestatic(u16),
    invokeinterface(u16, u8),
    invokedynamic(u16),
    bipush(u8),
    sipush(u16),

    new(u16),
    anewarray(u16),
    newarray(u8),
    arraylength,

    // Integer Operand codes
    istore_(u8),
    iconst_(i32),
    iload_(u8),

    // Array Operand Codes
    iaload,
    baload,
    aaload,
    iastore,
    aastore,
    castore,

    fstore_(u8),
    fconst_(f32),
    fload_(u8),

    astore_(u8),
    aconst_null,
    aload_(u8),

    lstore_(u8),
    lconst_(u64),
    lload_(u8),

    dstore_(u8),
    dconst_(f64),
    dload_(u8),

    checkcast(u16),
    i2f,
    i2l,

    putfield(u16),
    getfield(u16),

    putstatic(u16),

    if_icmp(CmpConditions, u16),
    if_cond(CmpConditions, u16),
    if_acmp(CmpConditions, u16),
    if_null(u16),
    if_notnull(u16),
    lcmp,

    isub,
    iand,
    iadd,
    imul,
    ixor,
    iushr,
    idiv,
    iinc(u8, i8),

    goto(u16),

    monitorenter,
    monitorexit,

    instanceof(u16),

    athrow,

    lreturn,
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
            0x5c => OpCodes::dup2,
            0x5a => OpCodes::dup_x1,
            0x57 => OpCodes::pop,
            0xb2 => OpCodes::getstatic(reader.read()?),
            0x12 => OpCodes::ldc(reader.read()?),
            0x13 => OpCodes::ldc_w(reader.read()?),
            0x14 => OpCodes::ldc2_w(reader.read()?),
            0xb6 => OpCodes::invokevirtual(reader.read()?),
            0xb7 => OpCodes::invokespecial(reader.read()?),
            0xb8 => OpCodes::invokestatic(reader.read()?),
            0xb9 => {
                let index = reader.read()?;
                let count = reader.read()?;

                assert!(
                    reader.read::<u8>()? == 0,
                    "The last byte in invokeinterface must be a '0'"
                );

                OpCodes::invokeinterface(index, count)
            }
            0xba => {
                let index = reader.read()?;

                assert!(
                    reader.read::<u16>()? == 0,
                    "The third and fourth operand bytes for invokedynamic must be '0'"
                );
                OpCodes::invokedynamic(index)
            }
            0x10 => OpCodes::bipush(reader.read()?),
            0x11 => OpCodes::sipush(reader.read()?),

            0xbb => OpCodes::new(reader.read()?),
            0xbd => OpCodes::anewarray(reader.read()?),
            0xbc => OpCodes::newarray(reader.read()?),
            0xbe => OpCodes::arraylength,

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
            0x8 => OpCodes::iconst_(5),

            0x15 => OpCodes::iload_(reader.read()?),
            0x1a => OpCodes::iload_(0),
            0x1b => OpCodes::iload_(1),
            0x1c => OpCodes::iload_(2),
            0x1d => OpCodes::iload_(3),

            0x2e => OpCodes::iaload,
            0x33 => OpCodes::baload,
            0x32 => OpCodes::aaload,
            0x4f => OpCodes::iastore,
            0x53 => OpCodes::aastore,
            0x55 => OpCodes::castore,

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

            0x37 => OpCodes::lstore_(reader.read()?),
            0x3f => OpCodes::lstore_(0),
            0x40 => OpCodes::lstore_(1),
            0x41 => OpCodes::lstore_(2),
            0x42 => OpCodes::lstore_(3),

            0x9 => OpCodes::lconst_(0),
            0xa => OpCodes::lconst_(1),

            0x16 => OpCodes::lload_(reader.read()?),
            0x1e => OpCodes::lload_(0),
            0x1f => OpCodes::lload_(1),
            0x20 => OpCodes::lload_(2),
            0x21 => OpCodes::lload_(3),

            0x39 => OpCodes::dstore_(reader.read()?),
            0x47 => OpCodes::dstore_(0),
            0x48 => OpCodes::dstore_(1),
            0x49 => OpCodes::dstore_(2),
            0x4a => OpCodes::dstore_(3),

            0xe => OpCodes::dconst_(0.0),
            0xf => OpCodes::dconst_(1.0),

            0x18 => OpCodes::dload_(reader.read()?),
            0x26 => OpCodes::dload_(0),
            0x27 => OpCodes::dload_(1),
            0x28 => OpCodes::dload_(2),
            0x29 => OpCodes::dload_(3),

            0xc0 => OpCodes::checkcast(reader.read()?),
            0x86 => OpCodes::i2f,
            0x85 => OpCodes::i2l,

            0xb5 => OpCodes::putfield(reader.read()?),
            0xb4 => OpCodes::getfield(reader.read()?),

            0xb3 => OpCodes::putstatic(reader.read()?),

            0x9f => OpCodes::if_icmp(CmpConditions::Equal, reader.read()?),
            0xa0 => OpCodes::if_icmp(CmpConditions::NotEqual, reader.read()?),
            0xa1 => OpCodes::if_icmp(CmpConditions::LessThan, reader.read()?),
            0xa4 => OpCodes::if_icmp(CmpConditions::LessOrEqual, reader.read()?),
            0xa3 => OpCodes::if_icmp(CmpConditions::GreaterThan, reader.read()?),
            0xa2 => OpCodes::if_icmp(CmpConditions::GreaterOrEqual, reader.read()?),

            0x99 => OpCodes::if_cond(CmpConditions::Equal, reader.read()?),
            0x9a => OpCodes::if_cond(CmpConditions::NotEqual, reader.read()?),
            0x9b => OpCodes::if_cond(CmpConditions::LessThan, reader.read()?),
            0x9e => OpCodes::if_cond(CmpConditions::LessOrEqual, reader.read()?),
            0x9d => OpCodes::if_cond(CmpConditions::GreaterThan, reader.read()?),
            0x9c => OpCodes::if_cond(CmpConditions::GreaterOrEqual, reader.read()?),

            0xa5 => OpCodes::if_acmp(CmpConditions::Equal, reader.read()?),
            0xa6 => OpCodes::if_acmp(CmpConditions::NotEqual, reader.read()?),

            0xc6 => OpCodes::if_null(reader.read()?),
            0xc7 => OpCodes::if_notnull(reader.read()?),

            0x94 => OpCodes::lcmp,

            0x64 => OpCodes::isub,
            0x7e => OpCodes::iand,
            0x60 => OpCodes::iadd,
            0x68 => OpCodes::imul,
            0x82 => OpCodes::ixor,
            0x7c => OpCodes::iushr,
            0x6c => OpCodes::idiv,
            0x84 => OpCodes::iinc(reader.read()?, reader.read()?),

            0xa7 => OpCodes::goto(reader.read()?),

            0xc2 => OpCodes::monitorenter,
            0xc3 => OpCodes::monitorexit,

            0xc1 => OpCodes::instanceof(reader.read()?),

            0xbf => OpCodes::athrow,

            0xad => OpCodes::lreturn,
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
