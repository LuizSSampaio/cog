use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum OpCode {
    Constant,
    Negate,
    Add,
    Subtract,
    Multiply,
    Divide,
    Return,
}

impl TryFrom<u8> for OpCode {
    type Error = OpCodeError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            x if x == OpCode::Constant as u8 => Ok(OpCode::Constant),
            x if x == OpCode::Negate as u8 => Ok(OpCode::Negate),
            x if x == OpCode::Add as u8 => Ok(OpCode::Add),
            x if x == OpCode::Subtract as u8 => Ok(OpCode::Subtract),
            x if x == OpCode::Multiply as u8 => Ok(OpCode::Multiply),
            x if x == OpCode::Divide as u8 => Ok(OpCode::Divide),
            x if x == OpCode::Return as u8 => Ok(OpCode::Return),
            _ => Err(OpCodeError::InvalidOpCode(value)),
        }
    }
}

#[derive(Debug, Error)]
pub enum OpCodeError {
    #[error("Invalid OpCode: {0}")]
    InvalidOpCode(u8),
}
