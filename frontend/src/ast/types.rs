use std::sync::mpsc::Sender;

use crate::ast::traits::{GetType, IsCompatible, StringToType};

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
#[derive(PartialEq, Debug)]
pub enum UVType {
    Int,
    Float,
    String,
    Boolean,
    Null,

    Union(Vec<UVType>),
}

impl IsCompatible for UVType {
    fn is_compatible_with(&self, other: &UVType) -> bool {
        if self == other {
            return true;
        }

        match (self, other) {
            (_, UVType::Union(types)) => types.iter().any(|t| self.is_compatible_with(t)),
            (UVType::Union(types), _) => types.iter().any(|t| t.is_compatible_with(other)),

            _ => false,
        }
    }
}

// -------------------- String-Type conversion --------------
impl StringToType for String {
    fn str_to_uvtype(&self) -> Option<UVType> {
        match self.as_str() {
            "int" => Some(UVType::Int),
            "float" => Some(UVType::Float),
            "str" => Some(UVType::String),
            "bool" => Some(UVType::Boolean),
            "null" => Some(UVType::Null),
            _ => None,
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

#[cfg(test)]
mod tests {
    use crate::ast::{
        traits::{IsCompatible, StringToType},
        types::UVType,
    };

    #[test]
    fn parse_type() {
        assert_eq!(String::from("int").str_to_uvtype(), Some(UVType::Int));
        assert_eq!(String::from("bool").str_to_uvtype(), Some(UVType::Boolean));
        assert_eq!(String::from("float").str_to_uvtype(), Some(UVType::Float));
        assert_eq!(String::from("null").str_to_uvtype(), Some(UVType::Null));
        assert_eq!(String::from("str").str_to_uvtype(), Some(UVType::String));

        assert_eq!(String::from("unknown").str_to_uvtype(), None);
    }

    #[test]
    fn type_compatible_with() {
        assert_eq!(
            UVType::Union(vec![UVType::Int, UVType::Null]).is_compatible_with(&UVType::Null),
            true
        );

        assert_eq!(
            UVType::Int.is_compatible_with(&UVType::Union(vec![UVType::Int, UVType::Null])),
            true
        );

        assert_eq!(UVType::Int.is_compatible_with(&UVType::Boolean), false);
    }
}
