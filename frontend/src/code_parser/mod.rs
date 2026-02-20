pub mod iterator;
pub mod types;

use crate::code_parser::types::ParseBody;
use crate::code_parser::{iterator::Iter, types::ParseNode};
use crate::errors::parser_error::ParserError;
use regex::Regex;

#[derive(Debug)]
enum ParseState {
    None,
    TagName,
    TagBody,
    ClosingTag,
    ExtraParam,
    AfterClosingTag,
}

pub struct CodeParser {
    iter: Iter<char>,
    buffer: String,
}

impl CodeParser {
    pub fn new(code: String) -> Self {
        let mut _code = Self::clear_comments(code);
        Self {
            iter: Iter::from(_code.chars()),
            buffer: String::new(),
        }
    }

    fn clear_comments(code: String) -> String {
        let re = Regex::new(r"(?s)<!--.*?-->").unwrap();
        re.replace_all(&code, "").to_string()
    }

    pub fn parse(&mut self) -> ParseNode {
        let mut parse_state = ParseState::None;
        let mut node = ParseNode {
            self_closing: false,
            name: String::new(),
            extra_param: String::new(),
            children: Vec::new(),
        };

        let mut closing_tag = String::new();
        while let Some(char) = self.iter.next() {
            match char {
                '<' => match parse_state {
                    ParseState::None => parse_state = ParseState::TagName,
                    ParseState::TagBody => {
                        self.buffer = self.buffer.trim().to_owned();

                        // If string buffer not empty
                        if !self.buffer.is_empty() {
                            node.children.push(ParseBody::String(self.buffer.clone()));
                            self.buffer.clear();
                        }

                        // If next tag is closing for current
                        if let Some('/') = self.iter.peek() {
                            self.iter.next(); // Consume '/'
                            parse_state = ParseState::ClosingTag;
                        } else {
                            self.iter.step_back(); // Return iter to `<` char
                            // Recursively parse nested tags
                            node.children.push(ParseBody::Node(Box::new(self.parse())));
                        }
                    }
                    _ => ParserError::error("Unexpected `<` tag", &mut self.iter),
                },
                '>' => match parse_state {
                    ParseState::TagName | ParseState::ExtraParam => {
                        parse_state = ParseState::TagBody
                    }
                    ParseState::ClosingTag | ParseState::AfterClosingTag => {
                        if node.name != closing_tag {
                            ParserError::error(
                                &format!(
                                    "Unexpected closing tag: </{}>. Expected </{}>",
                                    closing_tag, node.name
                                ),
                                &mut self.iter,
                            )
                        }
                        closing_tag.clear();
                        return node;
                    }
                    _ => ParserError::error("Unexpected `>` tag", &mut self.iter),
                },
                '/' => {
                    if let Some('>') = self.iter.peek() {
                        node.self_closing = true;
                        self.iter.next();

                        if node.name.is_empty() {
                            ParserError::error("Self-closing tag without name", &mut self.iter);
                        }
                        return node;
                    }
                }
                char if !char.is_whitespace() => match parse_state {
                    ParseState::TagName => node.name.push(char),
                    ParseState::ExtraParam => node.extra_param.push(char),
                    ParseState::ClosingTag => closing_tag.push(char),
                    ParseState::TagBody => self.buffer.push(char),
                    _ => ParserError::error("Unexpected literal", &mut self.iter),
                },
                char if char.is_whitespace() => match parse_state {
                    ParseState::TagName => parse_state = ParseState::ExtraParam,
                    ParseState::TagBody => self.buffer.push(char),
                    ParseState::ClosingTag => parse_state = ParseState::AfterClosingTag,
                    _ => {} // Just a whitespace
                },
                _ => {}
            }
        }

        ParserError::error("Unexpected EOF", &mut self.iter);
    }
}
