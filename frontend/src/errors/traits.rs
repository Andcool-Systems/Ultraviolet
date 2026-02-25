use crate::types::Span;

pub trait Positional {
    /// Get associated Span
    fn get_span(&self) -> Span;
}
