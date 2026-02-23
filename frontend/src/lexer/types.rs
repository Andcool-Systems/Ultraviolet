use crate::types::Span;

#[derive(Debug, Clone, PartialEq)]
pub enum UVLexerTokens {
    OpeningAngleBracket,
    ClosingAngleBracket,
    SelfClosingAngleBracket,  // />
    OpeningAngleBracketSlash, // </

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
            UVLexerTokens::Literal(str) => format!("[Literal \"{}\"]", str),
            UVLexerTokens::RawString(str) => format!("[Raw string \"{}\"]", str),
            UVLexerTokens::Unknown(ch) => ch.to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct UVToken {
    pub token: UVLexerTokens,
    pub span: Span,
}

#[derive(PartialEq)]
pub enum LexerParseState {
    Default,
    ParsingRawStringLiteral(Option<String>),
}
