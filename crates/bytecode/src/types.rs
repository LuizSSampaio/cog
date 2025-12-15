use std::fmt::Display;

use thiserror::Error;

use crate::values::Value;

/// Hexadecimals with this template are Types 0x2_
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Type {
    Int = 0x20,
    Float = 0x21,
    Bool = 0x22,
    Str = 0x23,
    Char = 0x24,
}

impl From<Value> for Type {
    fn from(value: Value) -> Self {
        match value {
            Value::Int(_) => Self::Int,
            Value::Float(_) => Self::Float,
            Value::Bool(_) => Self::Bool,
            Value::Str(_) => Self::Str,
            Value::Char(_) => Self::Char,
        }
    }
}

impl TryFrom<u8> for Type {
    type Error = TypeError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x20 => Ok(Type::Int),
            0x21 => Ok(Type::Float),
            0x22 => Ok(Type::Bool),
            0x23 => Ok(Type::Str),
            0x24 => Ok(Type::Char),
            _ => Err(TypeError::InvalidType(value)),
        }
    }
}

impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Type::Int => write!(f, "Int"),
            Type::Float => write!(f, "Float"),
            Type::Bool => write!(f, "Bool"),
            Type::Str => write!(f, "String"),
            Type::Char => write!(f, "Char"),
        }
    }
}

#[derive(Debug, Error)]
pub enum TypeError {
    #[error("Invalid Type: {0}")]
    InvalidType(u8),
}
