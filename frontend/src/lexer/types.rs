#[derive(Debug, Clone)]
pub enum UVLexerTokens {
    OpeningAngleBracket,
    ClosingAngleBracket,
    SelfClosingAngleBracket,  // />
    OpeningAngleBracketSlash, // </
    Slash,

    Literal(String),
    RawString(String),
}

#[derive(Debug, Clone)]
pub struct UVToken {
    pub token: UVLexerTokens,
    pub start: usize,
    pub end: usize,
}

#[derive(PartialEq)]
pub enum LexerParseState {
    Default,
    ParsingLiteral,
    ParsingRawStringLiteral,
}

pub enum RawStringTagType {
    Opening,
    Closing,
}
