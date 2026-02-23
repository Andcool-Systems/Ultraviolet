use crate::{
    errors::{ParseError, traits::Positional},
    types::SourceFile,
};
use colored::Colorize;

pub trait ErrorRenderer {
    fn display_with_source(&self, source: &SourceFile) -> String;
}

impl ErrorRenderer for ParseError {
    fn display_with_source(&self, source: &SourceFile) -> String {
        let (line, col) = source.get_line_col(self.get_span().clone());

        format!(
            "\n{}: {}",
            format!("{}:{}:{}", source.path.to_string_lossy(), line + 1, col + 1).red(),
            self.message
        )
    }
}

