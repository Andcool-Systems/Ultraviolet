use std::char;

use crate::{
    iterator::Iter,
    lexer::types::{LexerParseState, RawStringTagType, UVLexerTokens, UVToken},
};

mod types;

pub struct Lexer {
    iter: Iter<char>,
    tokens: Vec<UVToken>,

    buffer: String,
    parse_state: LexerParseState,

    token_start: usize,

    /// Array with start index of each line
    lines_map: Vec<usize>,
}

impl Lexer {
    pub fn new(input_code: String) -> Lexer {
        Self {
            iter: Iter::from(input_code.chars()),
            tokens: Vec::new(),
            buffer: String::new(),
            parse_state: LexerParseState::Default,
            token_start: 0,
            lines_map: std::iter::once(0)
                .chain(
                    input_code
                        .char_indices()
                        .filter(|(_, c)| *c == '\n')
                        .map(|(i, c)| i + c.len_utf8()),
                )
                .collect(),
        }
    }

    pub fn parse(&mut self) -> Vec<UVToken> {
        while let Some(ch) = self.iter.next() {
            let mut iteration_buffer = Vec::<UVToken>::new();
            match ch {
                '<' | '>' | '/' if self.parse_state != LexerParseState::ParsingRawStringLiteral => {
                    match self.finish_consuming_literal(true) {
                        Some(str) => iteration_buffer.push(UVToken {
                            token: UVLexerTokens::Literal(str),
                            start: self.token_start,
                            end: self.iter.pos,
                        }),
                        _ => {}
                    }

                    match ch {
                        '<' => {
                            self.token_start = self.iter.pos - 1;
                            if self.iter.peek(None) == Some('/') {
                                self.iter.next(); // Consume '/'
                                iteration_buffer.push(UVToken {
                                    token: UVLexerTokens::OpeningAngleBracketSlash,
                                    start: self.token_start,
                                    end: self.iter.pos,
                                });
                            } else {
                                iteration_buffer.push(UVToken {
                                    token: UVLexerTokens::OpeningAngleBracket,
                                    start: self.token_start,
                                    end: self.iter.pos,
                                });
                            }

                            match self.check_raw_str_tag() {
                                Some(RawStringTagType::Opening) => {
                                    self.parse_state = LexerParseState::ParsingRawStringLiteral;
                                    iteration_buffer.extend([
                                        UVToken {
                                            token: UVLexerTokens::Literal("str".to_string()),
                                            start: self.iter.pos - 4,
                                            end: self.iter.pos - 1,
                                        },
                                        UVToken {
                                            token: UVLexerTokens::ClosingAngleBracket,
                                            start: self.iter.pos - 1,
                                            end: self.iter.pos,
                                        },
                                    ]);
                                }
                                _ => {}
                            }
                        }
                        '>' => {
                            self.token_start = self.iter.pos - 1;
                            iteration_buffer.push(UVToken {
                                token: UVLexerTokens::ClosingAngleBracket,
                                start: self.token_start,
                                end: self.iter.pos,
                            });
                        }
                        '/' => {
                            self.token_start = self.iter.pos - 1;
                            if self.iter.peek(None) == Some('>') {
                                self.iter.next(); // Consume '>'
                                iteration_buffer.push(UVToken {
                                    token: UVLexerTokens::SelfClosingAngleBracket,
                                    start: self.token_start,
                                    end: self.iter.pos,
                                });
                            } else {
                                iteration_buffer.push(UVToken {
                                    token: UVLexerTokens::Slash,
                                    start: self.token_start,
                                    end: self.iter.pos,
                                });
                            }
                        }
                        _ => {}
                    }
                }

                char if self.parse_state == LexerParseState::ParsingRawStringLiteral => {
                    self.buffer.push(char);

                    if char == '<' {
                        match self.check_raw_str_tag() {
                            Some(RawStringTagType::Closing) => {
                                self.buffer.pop(); // Remove '<' from buffer
                                match self.finish_consuming_literal(false) {
                                    Some(str) => {
                                        iteration_buffer.push(UVToken {
                                            token: UVLexerTokens::RawString(str),
                                            start: self.token_start,
                                            end: self.iter.pos - 6,
                                        });
                                    }
                                    _ => {}
                                }
                                iteration_buffer.extend([
                                    UVToken {
                                        token: UVLexerTokens::OpeningAngleBracketSlash,
                                        start: self.iter.pos - 6,
                                        end: self.iter.pos - 5,
                                    },
                                    UVToken {
                                        token: UVLexerTokens::Literal("str".to_string()),
                                        start: self.iter.pos - 5,
                                        end: self.iter.pos - 1,
                                    },
                                    UVToken {
                                        token: UVLexerTokens::ClosingAngleBracket,
                                        start: self.iter.pos - 1,
                                        end: self.iter.pos,
                                    },
                                ]);
                                self.parse_state = LexerParseState::Default;
                            }
                            _ => {}
                        }
                    }
                }

                char if !char.is_whitespace() => match self.parse_state {
                    LexerParseState::Default => {
                        self.parse_state = LexerParseState::ParsingLiteral;
                        self.token_start = self.iter.pos - 1;
                        self.buffer.push(char);
                    }
                    LexerParseState::ParsingLiteral => self.buffer.push(char),
                    _ => {}
                },

                char if char.is_whitespace() => match self.parse_state {
                    LexerParseState::ParsingLiteral => match self.finish_consuming_literal(true) {
                        Some(str) => iteration_buffer.push(UVToken {
                            token: UVLexerTokens::Literal(str),
                            start: self.token_start,
                            end: self.iter.pos - 1,
                        }),
                        _ => {}
                    },
                    _ => {}
                },

                _ => {}
            }

            self.tokens.extend(iteration_buffer);
        }

        if let Some(lit) = self.finish_consuming_literal(true) {
            self.tokens.push(UVToken {
                token: UVLexerTokens::Literal(lit),
                start: self.token_start,
                end: self.iter.pos,
            });
        }

        self.tokens.clone()
    }

    /// Returns buffered literal
    fn finish_consuming_literal(&mut self, trim: bool) -> Option<String> {
        let text = if trim {
            self.buffer.trim()
        } else {
            &self.buffer
        };

        let token = if text.is_empty() {
            None
        } else {
            Some(text.to_owned())
        };

        self.buffer.clear();
        self.parse_state = LexerParseState::Default;

        token
    }

    /// Check if iterator currently reach <str> tag
    fn check_raw_str_tag(&mut self) -> Option<RawStringTagType> {
        self.iter.step_back(); // For proper consuming '<'

        let buffer = &self.iter.vec[self.iter.pos..];
        if buffer.starts_with(&['<', 's', 't', 'r', '>']) {
            self.iter.pos += 5;
            return Some(RawStringTagType::Opening);
        }
        if buffer.starts_with(&['<', '/', 's', 't', 'r', '>']) {
            self.iter.pos += 6;
            return Some(RawStringTagType::Closing);
        }

        self.iter.next(); // If no tag found - return iter to initial pos
        None
    }

    /// Get indexes of each line starts
    pub fn get_lines_indexes(&self) -> Vec<usize> {
        self.lines_map.clone()
    }
}
