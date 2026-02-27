use std::sync::mpsc::Sender;

use crate::ast::traits::{GetType, IsCompatible};

/// Typed value container
pub enum UVValue {
    Int(i64),
    Float(f64),
    String(String),
    Boolean(bool),
    Null,
}

impl GetType for UVValue {
    fn get_type(&self, _scope: Option<usize>) -> UVType {
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
#[derive(PartialEq)]
pub enum UVType {
    Int,
    Float,
    String,
    Boolean,
    Null,

    Union(Vec<UVType>),
}

impl IsCompatible for UVType {
    fn is_compatible(&self, other: &UVType) -> bool {
        if self == other {
            return true;
        }

        match (self, other) {
            (_, UVType::Union(types)) => types.iter().any(|t| self.is_compatible(t)),
            (UVType::Union(types), _) => types.iter().any(|t| t.is_compatible(other)),

            _ => false,
        }
    }
}

// ---------------
pub enum Symbol {
    /// Primitive type
    Primitive(UVValue),

    /// Name of the variable in scope
    Variable(String),
}

impl GetType for Symbol {
    fn get_type(&self, scope: Option<usize>) -> UVType {
        match self {
            Self::Primitive(val) => val.get_type(None),
            // Scope-based search of the final primitive
            Self::Variable(var) => todo!(),
        }
    }
}
