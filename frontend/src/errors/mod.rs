use std::fmt;

use crate::{errors::traits::Positional, types::Span};

pub mod error_renderer;
pub mod traits;

/// Simple parse error
pub struct SpannedError {
    message: String,
    span: Span,
}

impl SpannedError {
    /// Create new parse error
    pub fn new(message: impl Into<String>, span: Span) -> Self {
        Self {
            message: message.into(),
            span,
        }
    }
}

impl Positional for SpannedError {
    fn get_span(&self) -> Span {
        self.span.clone()
    }
}

impl fmt::Debug for SpannedError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SpannedError")
            .field("message", &self.message)
            .field("span", &self.span)
            .finish()
    }
}

impl fmt::Display for SpannedError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SpannedError")
            .field("message", &self.message)
            .field("span", &self.span)
            .finish()
    }
}

impl std::error::Error for SpannedError {}
