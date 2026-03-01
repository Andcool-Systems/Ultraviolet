use crate::types::{Span, TypeWithSpan};

#[derive(Debug, Clone, PartialEq)]
pub struct UVParseNode {
    pub name: String,
    pub children: Vec<UVParseBody>,

    pub self_closing: bool,
    pub extra_param: String,

    pub span: Span,
}

impl UVParseNode {
    /// Get count of children
    pub fn children_len(&self) -> usize {
        self.children.len()
    }

    /// Get inner TAG child by name
    pub fn get_child_by_name(&self, name: &str) -> Option<&UVParseNode> {
        self.children.iter().find_map(|ch| match ch {
            UVParseBody::Tag(node) if node.name == name => Some(node.as_ref()),
            _ => None,
        })
    }

    /// Get first inner literal
    pub fn get_inner_literal(&self) -> Option<&TypeWithSpan<String>> {
        self.children.iter().find_map(|ch| match ch {
            UVParseBody::String(literal) => Some(literal),
            _ => None,
        })
    }

    /// Get inner TAG at provided index
    pub fn get_child_node(&self, pos: usize) -> Option<&UVParseNode> {
        match self.children.get(pos) {
            Some(UVParseBody::Tag(child)) => Some(child),
            _ => None,
        }
    }

    /// Check if all children is literals
    pub fn all_literals(&self) -> bool {
        self.children
            .iter()
            .all(|ch| matches!(ch, UVParseBody::String(_)))
    }

    /// Check if all children is tags
    pub fn all_tags(&self) -> bool {
        self.children
            .iter()
            .all(|ch| matches!(ch, UVParseBody::Tag(_)))
    }
}

// -------------------------------------

#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)] // TODO: Remove
pub enum UVParseBody {
    String(TypeWithSpan<String>),
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
