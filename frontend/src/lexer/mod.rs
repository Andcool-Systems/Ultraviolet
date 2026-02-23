use std::char;

use crate::{
    iterator::Iter,
    lexer::types::{LexerParseState, RawStringTagType, UVLexerTokens, UVToken},
};
pub mod types;

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
                            if self.check_comment_and_consume() {
                                continue;
                            }

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

                char if Self::is_valid_literal(char) => match self.parse_state {
                    LexerParseState::Default => {
                        self.parse_state = LexerParseState::ParsingLiteral;
                        self.token_start = self.iter.pos - 1;
                        self.buffer.push(char);
                    }
                    LexerParseState::ParsingLiteral => self.buffer.push(char),
                    _ => {}
                },

                char if !Self::is_valid_literal(char) => match self.parse_state {
                    LexerParseState::ParsingLiteral => match self.finish_consuming_literal(true) {
                        Some(str) => {
                            iteration_buffer.push(UVToken {
                                token: UVLexerTokens::Literal(str),
                                start: self.token_start,
                                end: self.iter.pos - 1,
                            });
                            self.iter.step_back();
                        }
                        _ => {}
                    },
                    LexerParseState::Default if !char.is_whitespace() => {
                        iteration_buffer.push(UVToken {
                            token: UVLexerTokens::Unknown(char),
                            start: self.iter.pos - 1,
                            end: self.iter.pos,
                        })
                    }
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

    fn check_comment_and_consume(&mut self) -> bool {
        self.iter.step_back();

        if !&self.iter.vec[self.iter.pos..].starts_with(&['<', '!', '-', '-']) {
            self.iter.next();
            return false;
        }

        while !&self.iter.vec[self.iter.pos..].starts_with(&['-', '-', '>']) {
            self.iter.next();
        }

        self.iter.pos += 3;
        return true;
    }

    fn is_valid_literal(c: char) -> bool {
        matches!(c, 'a'..='z' | 'A'..='Z' | '0'..='9' | '.' | ',' | '_')
    }

    /// Get indexes of each line starts
    pub fn get_lines_indexes(&self) -> Vec<usize> {
        self.lines_map.clone()
    }
}

#[cfg(test)]
mod tests {
    use crate::lexer::{Lexer, types::UVLexerTokens};

    fn get_tokens(code: &str) -> Vec<UVLexerTokens> {
        Lexer::new(code.to_owned())
            .parse()
            .into_iter()
            .map(|t| t.token)
            .collect::<Vec<UVLexerTokens>>()
    }

    #[test]
    fn parse_simple() {
        assert_eq!(
            get_tokens("<main><test /></main>"),
            [
                UVLexerTokens::OpeningAngleBracket,
                UVLexerTokens::Literal("main".to_owned()),
                UVLexerTokens::ClosingAngleBracket,
                UVLexerTokens::OpeningAngleBracket,
                UVLexerTokens::Literal("test".to_owned()),
                UVLexerTokens::SelfClosingAngleBracket,
                UVLexerTokens::OpeningAngleBracketSlash,
                UVLexerTokens::Literal("main".to_owned()),
                UVLexerTokens::ClosingAngleBracket
            ]
        )
    }

    #[test]
    fn parse_inner_literal() {
        assert_eq!(
            get_tokens("<main>test</main>"),
            [
                UVLexerTokens::OpeningAngleBracket,
                UVLexerTokens::Literal("main".to_owned()),
                UVLexerTokens::ClosingAngleBracket,
                UVLexerTokens::Literal("test".to_owned()),
                UVLexerTokens::OpeningAngleBracketSlash,
                UVLexerTokens::Literal("main".to_owned()),
                UVLexerTokens::ClosingAngleBracket
            ]
        )
    }

    #[test]
    fn parse_unknown() {
        assert_eq!(
            get_tokens("<main>?</main>"),
            [
                UVLexerTokens::OpeningAngleBracket,
                UVLexerTokens::Literal("main".to_owned()),
                UVLexerTokens::ClosingAngleBracket,
                UVLexerTokens::Unknown('?'),
                UVLexerTokens::OpeningAngleBracketSlash,
                UVLexerTokens::Literal("main".to_owned()),
                UVLexerTokens::ClosingAngleBracket
            ]
        )
    }

    #[test]
    fn parse_comments() {
        assert_eq!(
            get_tokens("<main><!-- this is a comment! --></main>"),
            [
                UVLexerTokens::OpeningAngleBracket,
                UVLexerTokens::Literal("main".to_owned()),
                UVLexerTokens::ClosingAngleBracket,
                UVLexerTokens::OpeningAngleBracketSlash,
                UVLexerTokens::Literal("main".to_owned()),
                UVLexerTokens::ClosingAngleBracket
            ]
        )
    }

    #[test]
    fn parse_raw_str() {
        assert_eq!(
            get_tokens("<str> Random content <null /> </str>"),
            [
                UVLexerTokens::OpeningAngleBracket,
                UVLexerTokens::Literal("str".to_owned()),
                UVLexerTokens::ClosingAngleBracket,
                UVLexerTokens::RawString(" Random content <null /> ".to_owned()),
                UVLexerTokens::OpeningAngleBracketSlash,
                UVLexerTokens::Literal("str".to_owned()),
                UVLexerTokens::ClosingAngleBracket
            ]
        )
    }
}
