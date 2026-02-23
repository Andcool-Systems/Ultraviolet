use crate::types::Span;

#[derive(Debug, Clone, PartialEq)]
pub struct UVParseNode {
    pub name: String,
    pub children: Vec<UVParseBody>,

    pub self_closing: bool,
    pub extra_param: String,

    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub struct UVParseLiteral {
    pub value: String,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)] // TODO: Remove
pub enum UVParseBody {
    String(UVParseLiteral),
    Tag(Box<UVParseNode>),
}

#[derive(Debug)]
pub enum UVParseState {
    Unknown,
    TagName,
    TagBody,
    ExtraParam,
    ClosingAngleBracketOpeningTag,
    ClosingAngleBracketClosingTag,
    ClosingTagName,
}
