#[allow(non_camel_case_types)]
#[derive(Debug)]
pub enum OpCodes {
    OpCodeError(u8),
    nop,
    dup,
    getstatic,
    ldc,
    invokevirtual,
    invokespecial,
    invokestatic,
    bipush,
    sipush,

    new,
    anewarray,

    istore_(usize),
    iconst_(i32),
    iload_(usize),

    fstore_(usize),
    fconst_(f32),
    fload_(usize),

    Return,
}

impl From<u8> for OpCodes {
    fn from(v: u8) -> Self {
        match v {
            0x00 => OpCodes::nop,
            0x59 => OpCodes::dup,
            0xb2 => OpCodes::getstatic,
            0x12 => OpCodes::ldc,
            0xb6 => OpCodes::invokevirtual,
            0xb7 => OpCodes::invokespecial,
            0xb8 => OpCodes::invokestatic,
            0x10 => OpCodes::bipush,
            0x11 => OpCodes::sipush,

            0xbb => OpCodes::new,
            0xbd => OpCodes::anewarray,

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

            0x1a => OpCodes::iload_(0),
            0x1b => OpCodes::iload_(1),
            0x1c => OpCodes::iload_(2),
            0x1d => OpCodes::iload_(3),

            0x43 => OpCodes::fstore_(0),
            0x44 => OpCodes::fstore_(1),
            0x45 => OpCodes::fstore_(2),
            0x46 => OpCodes::fstore_(3),

            0xb => OpCodes::fconst_(0.0),
            0xc => OpCodes::fconst_(1.0),
            0xd => OpCodes::fconst_(2.0),

            0x22 => OpCodes::fload_(0),
            0x23 => OpCodes::fload_(1),
            0x24 => OpCodes::fload_(2),
            0x25 => OpCodes::fload_(3),

            0xb1 => OpCodes::Return,
            _ => OpCodes::OpCodeError(v),
        }
    }
}
