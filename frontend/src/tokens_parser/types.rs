use crate::types::Span;

#[derive(Debug, Clone, PartialEq)]
pub struct UVParseNode {
    pub name: String,
    pub children: Vec<UVParseBody>,

    pub self_closing: bool,
    pub extra_param: String,

    pub span: Span,
}

impl UVParseNode {
    /// Get inner TAG child by name
    pub fn get_child_by_name(&self, name: &str) -> Option<&UVParseNode> {
        self.children.iter().find_map(|ch| match ch {
            UVParseBody::Tag(node) if node.name == name => Some(node.as_ref()),
            _ => None,
        })
    }

    /// Get first inner literal
    pub fn get_inner_literal(&self) -> Option<&UVParseLiteral> {
        self.children.iter().find_map(|ch| match ch {
            UVParseBody::String(literal) => Some(literal),
            _ => None,
        })
    }
}

// -------------------------------------

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
