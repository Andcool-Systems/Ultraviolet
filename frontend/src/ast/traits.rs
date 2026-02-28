use crate::ast::types::UVType;

pub trait GetType {
    /// Get type of node / value
    /// TODO: Replace scope with real scope type
    fn get_type(&self, scope: Option<usize>) -> UVType;
}

pub trait IsCompatible {
    /// Check if left (current) type is compatible with right type
    fn is_compatible_with(&self, other: &UVType) -> bool;
}

pub trait StringToType {
    /**
    Convert string-representation to a Ultraviolet type

    Example:
    `String::from("int").str_to_uvtype();`
    */
    fn str_to_uvtype(&self) -> Option<UVType>;
}
