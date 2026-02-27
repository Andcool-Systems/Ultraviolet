use crate::ast::types::UVType;

pub trait GetType {
    /// Get type of node / value
    fn get_type(&self) -> UVType;
}
