#[derive(Debug, Clone, PartialEq)]
pub enum UVLexerTokens {
    OpeningAngleBracket,
    ClosingAngleBracket,
    SelfClosingAngleBracket,  // />
    OpeningAngleBracketSlash, // </
    Slash,

    Literal(String),
    RawString(String),

    Unknown(char),
}

impl ToString for UVLexerTokens {
    fn to_string(&self) -> String {
        match self {
            UVLexerTokens::OpeningAngleBracket => "<".to_owned(),
            UVLexerTokens::ClosingAngleBracket => ">".to_owned(),
            UVLexerTokens::SelfClosingAngleBracket => "/>".to_owned(),
            UVLexerTokens::OpeningAngleBracketSlash => "</".to_owned(),
            UVLexerTokens::Slash => "/".to_owned(),
            UVLexerTokens::Literal(str) => format!("[Literal \"{}\"]", str),
            UVLexerTokens::RawString(str) => format!("[Raw string \"{}\"]", str),
            UVLexerTokens::Unknown(ch) => ch.to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct UVToken {
    pub token: UVLexerTokens,
    pub start: usize,
    pub end: usize,
}

#[derive(PartialEq)]
pub enum LexerParseState {
    Default,
    ParsingRawStringLiteral(Option<String>),
}
