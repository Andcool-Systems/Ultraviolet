use crate::ast::types::UVType;

pub trait GetType {
    /// Get type of node / value
    /// TODO: Replace scope with real scope type
    fn get_type(&self, scope: Option<usize>) -> UVType;
}

pub trait IsCompatible {
    /// Check if left (current) type is compatible with right type
    fn is_compatible(&self, other: &UVType) -> bool;
}
