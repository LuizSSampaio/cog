use thiserror::Error;

use crate::types::{Type, TypeError};

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Value {
    Int(isize),
    Float(f64),
    Bool(bool),
    Str(String),
    Char(char),
}

macro_rules! impl_from_int {
    ($($t:ty), *) => {
        $(impl From<$t> for Value {
            fn from(value: $t) -> Self {
                Value::Int(value as isize)
            }
        })*
    };
}
impl_from_int!(
    i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize
);

macro_rules! impl_from_float {
    ($($t:ty), *) => {
        $(impl From<$t> for Value {
            fn from(value: $t) -> Self {
                Value::Float(value as f64)
            }
        })*
    };
}
impl_from_float!(f32, f64);

impl From<bool> for Value {
    fn from(value: bool) -> Self {
        Value::Bool(value)
    }
}

impl From<String> for Value {
    fn from(value: String) -> Self {
        Value::Str(value)
    }
}

impl From<&str> for Value {
    fn from(value: &str) -> Self {
        Value::Str(value.to_string())
    }
}

impl From<char> for Value {
    fn from(value: char) -> Self {
        Value::Char(value)
    }
}

impl TryFrom<Value> for isize {
    type Error = ValueError;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Int(i) => Ok(i),
            _ => Err(ValueError::InvalidConversion {
                from: Type::from(&value),
                to: Type::Int,
            }),
        }
    }
}

impl TryFrom<Value> for f64 {
    type Error = ValueError;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Float(f) => Ok(f),
            _ => Err(ValueError::InvalidConversion {
                from: Type::from(&value),
                to: Type::Float,
            }),
        }
    }
}

impl From<Value> for Vec<u8> {
    fn from(value: Value) -> Self {
        let mut buffer = Vec::new();
        buffer.push(Type::from(&value) as u8);

        match value {
            Value::Int(val) => buffer.extend_from_slice(&(val as i64).to_le_bytes()),
            Value::Float(val) => buffer.extend_from_slice(&val.to_le_bytes()),
            Value::Bool(val) => buffer.push(val as u8),
            Value::Str(val) => {
                let bytes = val.as_bytes();
                let len = bytes.len() as u32;
                buffer.extend_from_slice(&len.to_le_bytes());
                buffer.extend_from_slice(bytes);
            }
            Value::Char(val) => buffer.push(val as u8),
        }

        buffer
    }
}

impl TryFrom<Vec<u8>> for Value {
    type Error = ValueError;

    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        let Some(tag) = value.first() else {
            return Err(ValueError::NoTag);
        };
        let data_len = value.len() - 1;

        match Type::try_from(tag.to_owned())? {
            Type::Int => {
                if data_len != 8 {
                    return Err(ValueError::IncompatibleSize);
                }

                let mut slice = [0u8; 8];
                slice.copy_from_slice(&value[1..]);
                Ok(Value::Int(i64::from_le_bytes(slice) as isize))
            }
            Type::Float => {
                if data_len != 8 {
                    return Err(ValueError::IncompatibleSize);
                }

                let mut slice = [0u8; 8];
                slice.copy_from_slice(&value[1..]);
                Ok(Value::Float(f64::from_le_bytes(slice)))
            }
            Type::Bool => {
                if data_len != 1 {
                    return Err(ValueError::IncompatibleSize);
                }

                Ok(Value::Bool(value[1] != 0))
            }
            Type::Str => {
                if data_len < 4 {
                    return Err(ValueError::IncompatibleSize);
                }

                let mut len_slice = [0u8; 4];
                len_slice.copy_from_slice(&value[1..=4]);

                let len = u32::from_le_bytes(len_slice) as usize;
                if data_len != len + 4 {
                    return Err(ValueError::IncompatibleSize);
                }

                let str = String::from_utf8_lossy(&value[5..]);
                Ok(Value::Str(str.to_string()))
            }
            Type::Char => {
                if data_len != 1 {
                    return Err(ValueError::IncompatibleSize);
                }

                Ok(Value::Char(value[1] as char))
            }
        }
    }
}

#[derive(Debug, Error)]
pub enum ValueError {
    #[error("Invalid conversion between {from} and {to}")]
    InvalidConversion { from: Type, to: Type },
    #[error("Buffer don't has a type tag")]
    NoTag,
    #[error("Value size is incompatible with the received buffer size")]
    IncompatibleSize,
    #[error(transparent)]
    Type(#[from] TypeError),
}
