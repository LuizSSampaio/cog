use thiserror::Error;

/// Hexadecimals with this template are OpCodes 0x1_
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum OpCode {
    Constant = 0x10,
    Negate = 0x11,
    Add = 0x12,
    Subtract = 0x13,
    Multiply = 0x14,
    Divide = 0x15,
    Return = 0x16,
}

impl TryFrom<u8> for OpCode {
    type Error = OpCodeError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x10 => Ok(OpCode::Constant),
            0x11 => Ok(OpCode::Negate),
            0x12 => Ok(OpCode::Add),
            0x13 => Ok(OpCode::Subtract),
            0x14 => Ok(OpCode::Multiply),
            0x15 => Ok(OpCode::Divide),
            0x16 => Ok(OpCode::Return),
            _ => Err(OpCodeError::InvalidOpCode(value)),
        }
    }
}

#[derive(Debug, Error)]
pub enum OpCodeError {
    #[error("Invalid OpCode: {0}")]
    InvalidOpCode(u8),
}
