use crate::ast::traits::GetType;

/// Typed value container
pub enum UVValue {
    Int(i64),
    Float(f64),
    String(String),
    Boolean(bool),
    Null,
}

impl GetType for UVValue {
    fn get_type(&self) -> UVType {
        match self {
            UVValue::Int(_) => UVType::Int,
            UVValue::Float(_) => UVType::Float,
            UVValue::String(_) => UVType::String,
            UVValue::Boolean(_) => UVType::Boolean,
            UVValue::Null => UVType::Null,
        }
    }
}

/// Ultraviolet primitive types
pub enum UVType {
    Int,
    Float,
    String,
    Boolean,
    Null,

    Union(Vec<UVValue>),
}
