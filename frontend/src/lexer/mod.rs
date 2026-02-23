use std::char;

use crate::{
    iterator::Iter,
    lexer::types::{LexerParseState, UVLexerTokens, UVToken},
};
pub mod types;

const RAW_OPEN_LEN: usize = 5;
const RAW_CLOSE_LEN: usize = 6;

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
        while let Some(_) = self.iter.peek(None) {
            let iteration_buffer = match self.parse_state {
                LexerParseState::Default => self.lex_normal_mode(),
                LexerParseState::ParsingRawStringLiteral(_) => self.lex_raw_mode(),
            };

            self.tokens.extend(iteration_buffer);
        }

        let trim_end = if matches!(self.parse_state, LexerParseState::Default) {
            true
        } else {
            false
        };

        if let Some(lit) = self.finish_consuming_literal(trim_end) {
            let token = match self.parse_state {
                LexerParseState::Default => UVLexerTokens::Literal(lit),
                LexerParseState::ParsingRawStringLiteral(_) => UVLexerTokens::RawString(lit),
            };
            self.tokens.push(UVToken {
                token: token,
                start: self.token_start,
                end: self.iter.pos,
            });
        }
        self.tokens.clone()
    }

    fn lex_normal_mode(&mut self) -> Vec<UVToken> {
        let ch = self.iter.next().unwrap(); // This inwrap is potentially unreachable
        let mut iteration_buffer = Vec::<UVToken>::new();

        match ch {
            '<' | '>' | '/' => {
                match self.finish_consuming_literal(true) {
                    Some(str) => iteration_buffer.push(UVToken {
                        token: UVLexerTokens::Literal(str.clone()),
                        start: self.token_start,
                        end: self.iter.pos - 1,
                    }),
                    _ => {}
                }

                match ch {
                    '<' => {
                        if self.check_comment_and_consume() {
                            return iteration_buffer;
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

                        match self.check_opening_raw_str_tag() {
                            Some(key) => {
                                self.parse_state = LexerParseState::ParsingRawStringLiteral(key);
                                iteration_buffer.extend([
                                    UVToken {
                                        token: UVLexerTokens::Literal("str".to_string()),
                                        start: self.iter.pos - (RAW_OPEN_LEN - 1),
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
            char if Self::is_valid_literal(char) => {
                if self.buffer.is_empty() {
                    self.token_start = self.iter.pos - 1;
                }
                self.buffer.push(char);
            }

            char if !Self::is_valid_literal(char) => {
                if !self.buffer.is_empty() {
                    match self.finish_consuming_literal(true) {
                        Some(str) => {
                            iteration_buffer.push(UVToken {
                                token: UVLexerTokens::Literal(str),
                                start: self.token_start,
                                end: self.iter.pos - 1,
                            });
                            self.iter.step_back();
                        }
                        _ => {}
                    }
                } else if !char.is_whitespace() {
                    iteration_buffer.push(UVToken {
                        token: UVLexerTokens::Unknown(char),
                        start: self.iter.pos - 1,
                        end: self.iter.pos,
                    })
                }
            }

            _ => {}
        }

        iteration_buffer
    }

    fn lex_raw_mode(&mut self) -> Vec<UVToken> {
        let ch = self.iter.next().unwrap(); // This inwrap is potentially unreachable
        let mut iteration_buffer = Vec::<UVToken>::new();

        self.buffer.push(ch);

        if ch == '<' {
            match self.check_closing_raw_str_tag() {
                Some(true) => {
                    self.buffer.pop(); // Remove '<' from buffer
                    match self.finish_consuming_literal(false) {
                        Some(str) => {
                            iteration_buffer.push(UVToken {
                                token: UVLexerTokens::RawString(str),
                                start: self.token_start,
                                end: self.iter.pos - RAW_CLOSE_LEN,
                            });
                        }
                        _ => {}
                    }
                    iteration_buffer.extend([
                        UVToken {
                            token: UVLexerTokens::OpeningAngleBracketSlash,
                            start: self.iter.pos - RAW_CLOSE_LEN,
                            end: self.iter.pos - RAW_OPEN_LEN,
                        },
                        UVToken {
                            token: UVLexerTokens::Literal("str".to_string()),
                            start: self.iter.pos - RAW_OPEN_LEN,
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

        iteration_buffer
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
        token
    }

    /// Consume all symbols after <str- and before >
    fn consume_raw_str_label(&mut self) -> Option<String> {
        let mut buffer = String::new();
        while let Some(char) = self.iter.next() {
            if char == '>' {
                return Some(buffer);
            } else {
                buffer.push(char);
            }
        }
        None
    }

    /// Check if iterator currently reach <str-xx> tag
    fn check_opening_raw_str_tag(&mut self) -> Option<Option<String>> {
        let start_iter_pos = self.iter.pos;
        self.iter.step_back(); // For proper consuming '<'

        if self.iter.starts_with(&['<', 's', 't', 'r']) {
            self.iter.pos += 4;
            match self.iter.next() {
                Some('>') => return Some(None),
                Some('-') => return Some(self.consume_raw_str_label()),
                _ => {}
            }
        }

        self.iter.pos = start_iter_pos;
        None
    }

    /// Check if iterator currently reach </str-xx> tag
    fn check_closing_raw_str_tag(&mut self) -> Option<bool> {
        let start_iter_pos = self.iter.pos;
        self.iter.step_back(); // For proper consuming '<'

        if self.iter.starts_with(&['<', '/', 's', 't', 'r']) {
            self.iter.pos += 5;

            match self.iter.next() {
                Some('>')
                    if matches!(
                        self.parse_state,
                        LexerParseState::ParsingRawStringLiteral(None)
                    ) =>
                {
                    return Some(true);
                }
                Some('-') => {
                    let label = self.consume_raw_str_label();
                    if let Some(label) = label
                        && let LexerParseState::ParsingRawStringLiteral(Some(start_label)) =
                            &self.parse_state
                        && start_label.eq(&label)
                    {
                        return Some(true);
                    }
                }
                _ => {}
            }
        }

        self.iter.pos = start_iter_pos;
        None
    }

    fn check_comment_and_consume(&mut self) -> bool {
        self.iter.step_back();

        if !self.iter.starts_with(&['<', '!', '-', '-']) {
            self.iter.next();
            return false;
        }

        while let Some(_) = self.iter.next() {
            if self.iter.starts_with(&['-', '-', '>']) {
                self.iter.pos += 3;
                return true;
            }
        }
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
    use crate::lexer::{
        Lexer,
        types::{UVLexerTokens, UVToken},
    };

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
    fn unclosed_comment() {
        assert_eq!(
            get_tokens("<main><!-- this is an unclosed comment!</main>"),
            [
                UVLexerTokens::OpeningAngleBracket,
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

    #[test]
    fn parse_labeled_str() {
        assert_eq!(
            get_tokens("<str-test> Random content <str-123></str-123> <null /> </str-test>"),
            [
                UVLexerTokens::OpeningAngleBracket,
                UVLexerTokens::Literal("str".to_owned()),
                UVLexerTokens::ClosingAngleBracket,
                UVLexerTokens::RawString(
                    " Random content <str-123></str-123> <null /> ".to_owned()
                ),
                UVLexerTokens::OpeningAngleBracketSlash,
                UVLexerTokens::Literal("str".to_owned()),
                UVLexerTokens::ClosingAngleBracket
            ]
        )
    }

    #[test]
    fn parse_broken_raw_str() {
        assert_eq!(
            get_tokens("<str> Random content <null /> </str"),
            [
                UVLexerTokens::OpeningAngleBracket,
                UVLexerTokens::Literal("str".to_owned()),
                UVLexerTokens::ClosingAngleBracket,
                UVLexerTokens::RawString(" Random content <null /> </str".to_owned())
            ]
        )
    }

    #[test]
    fn test_indexes() {
        assert_eq!(
            Lexer::new("<main>test</main>".to_owned()).parse(),
            [
                UVToken {
                    token: UVLexerTokens::OpeningAngleBracket,
                    start: 0,
                    end: 1
                },
                UVToken {
                    token: UVLexerTokens::Literal("main".to_owned()),
                    start: 1,
                    end: 5
                },
                UVToken {
                    token: UVLexerTokens::ClosingAngleBracket,
                    start: 5,
                    end: 6
                },
                UVToken {
                    token: UVLexerTokens::Literal("test".to_owned()),
                    start: 6,
                    end: 10
                },
                UVToken {
                    token: UVLexerTokens::OpeningAngleBracketSlash,
                    start: 10,
                    end: 12
                },
                UVToken {
                    token: UVLexerTokens::Literal("main".to_owned()),
                    start: 12,
                    end: 16
                },
                UVToken {
                    token: UVLexerTokens::ClosingAngleBracket,
                    start: 16,
                    end: 17
                },
            ]
        )
    }
}
