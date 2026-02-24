use crate::{
    errors::{ParseError, traits::Positional},
    types::SourceFile,
};
use anyhow::Result;
use colored::Colorize;
use std::fmt::Write;

pub trait ErrorRenderer {
    fn render_error_line(&self, line: usize, col: usize, source: &SourceFile) -> String;
    fn display_with_source(&self, source: &SourceFile) -> String;
    fn render_extended(&self, source: &SourceFile) -> Result<String>;
}

impl ErrorRenderer for ParseError {
    fn render_error_line(&self, line: usize, col: usize, source: &SourceFile) -> String {
        format!("{}:{}:{}", source.path.to_string_lossy(), line + 1, col + 1)
    }

    fn display_with_source(&self, source: &SourceFile) -> String {
        if let Ok(str) = self.render_extended(source) {
            return str;
        }

        let (line, col) = source.get_line_col(self.get_span().clone());
        format!(
            "\n{}: {}",
            self.render_error_line(line, col, source).red(),
            self.message
        )
    }

    fn render_extended(&self, source: &SourceFile) -> Result<String> {
        let (line, col) = source.get_line_col(self.get_span().clone());
        let mut line_content = source.get_line_content(line)?;
        let original_len = line_content.len();

        line_content = line_content.trim_start().to_owned();
        let col_offsetted = col - (original_len - line_content.len());

        let error_line_link = self.render_error_line(line, col, source);

        let editor_line = line + 1;
        let line_no_len = editor_line.to_string().len();

        let mut output = String::new();
        writeln!(output, "{}: {}", "error".red(), self.message)?;
        writeln!(output, " --> {}", error_line_link)?;
        writeln!(output, " {} |", " ".repeat(line_no_len))?;
        writeln!(output, " {} | {}", editor_line, line_content)?;
        writeln!(
            output,
            " {} | {}{}",
            " ".repeat(line_no_len),
            " ".repeat(col_offsetted),
            "^".repeat(self.span.end - self.span.start)
        )?;

        Ok(output)
    }
}
