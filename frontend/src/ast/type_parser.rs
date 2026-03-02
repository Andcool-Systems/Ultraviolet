use crate::{
    ast::types::{ASTBlockType, UVType},
    errors::SpannedError,
    tokens_parser::types::UVParseNode,
};

/// Parse Ultraviolet type
pub fn parse_type(node: UVParseNode) -> Result<ASTBlockType, SpannedError> {
    Ok(ASTBlockType::Type(parse(node)?))
}

fn parse(node: UVParseNode) -> Result<UVType, SpannedError> {
    if !node.self_closing && node.name != "union" {
        return Err(SpannedError::new(
            "All type tags must be self-closing",
            node.span,
        ));
    }

    Ok(match node.name.as_str() {
        "int" => UVType::Int,
        "float" => UVType::Float,
        "str" => UVType::String,
        "bool" => UVType::Boolean,
        "null" => UVType::Null,
        "union" => parse_union(node)?,
        _ => {
            return Err(SpannedError::new(
                format!("Unknown type `{}`", node.name),
                node.span.clone(),
            ));
        }
    })
}

fn parse_union(node: UVParseNode) -> Result<UVType, SpannedError> {
    if !node.all_tags() {
        return Err(SpannedError::new(
            "All children inside union tag must be known types",
            node.span,
        ));
    }

    if node.children_len() == 0 {
        return Err(SpannedError::new("Union type cannot be empty", node.span));
    }

    if node.children_len() == 1 {
        return Ok(parse(node.get_child_node(0).unwrap().clone())?);
    }

    let types: Result<Vec<UVType>, SpannedError> = node
        .get_all_tags()
        .iter()
        .map(|ch| parse(ch.clone()))
        .collect();
    Ok(UVType::Union(types?))
}
