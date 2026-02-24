use anyhow::{Context, Result};
use std::{fs, path::Path};
pub struct SourceFile<'a> {
    pub path: &'a Path,
    pub code: String,
    pub line_starts: Vec<usize>,
}

impl<'a> SourceFile<'a> {
    pub fn load(path: &'a Path) -> Result<Self> {
        let code: String = fs::read_to_string(path)?;
        Ok(Self {
            path: path,
            code: code.clone(),
            line_starts: std::iter::once(0)
                .chain(
                    code.char_indices()
                        .filter(|(_, c)| *c == '\n')
                        .map(|(i, c)| i + c.len_utf8()),
                )
                .collect(),
        })
    }

    pub fn get_line_col(&self, span: Span) -> (usize, usize) {
        let line = Self::find_insert_position(&self.line_starts, span.start).unwrap_or(0);
        let column = span.start - self.line_starts[line];

        (line, column)
    }

    fn find_insert_position(arr: &[usize], target: usize) -> Option<usize> {
        if arr.is_empty() || target < arr[0] {
            return None;
        }

        match arr.binary_search(&target) {
            Ok(index) => Some(index),
            Err(index) => {
                if index > 0 {
                    Some(index - 1)
                } else {
                    None
                }
            }
        }
    }

    pub fn get_line_content(&self, line: usize) -> Result<String> {
        let line_index_start = self.line_starts.get(line).context("")?;
        let code_len = self.code.len();
        let line_index_end = self.line_starts.get(line + 1).unwrap_or(&code_len);

        let line_content = self
            .code
            .get(*line_index_start..*line_index_end)
            .context("")?
            .trim_end_matches("\n");

        Ok(line_content.to_string())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

impl Span {
    pub fn new(s: usize, e: usize) -> Self {
        Self { start: s, end: e }
    }
}

impl Default for Span {
    fn default() -> Self {
        Self { start: 0, end: 0 }
    }
}
