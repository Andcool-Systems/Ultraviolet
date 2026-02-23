use crate::types::Span;

pub trait Positional {
    fn get_span(&self) -> Span;
}
