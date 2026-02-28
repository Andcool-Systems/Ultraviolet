use crate::ast::types::UVType;

pub trait GetType {
    /// Get type of node / value
    fn get_type(&self) -> UVType;
}

pub trait GetTypeScope {
    /// Get symbol type from scope
    /// TODO: Replace scope with real scope type
    fn get_type_from_scope(&self, scope: Option<usize>) -> UVType;
}

pub trait IsAssignable {
    /// Returns `true` if `other` is a subtype of `self`.
    ///
    /// This defines assignability in the type system.
    /// A value of type `other` is assignable to `self` if every possible
    /// runtime value of `other` is valid for `self`.
    fn is_assignable_from(&self, other: &UVType) -> bool;
}

pub trait StringToType {
    /// Convert string-representation to a Ultraviolet type
    ///
    /// Example:
    /// `String::from("int").str_to_uvtype();`
    fn str_to_uvtype(&self) -> Option<UVType>;
}
