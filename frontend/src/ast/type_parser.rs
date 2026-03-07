use crate::{
    ast::{
        GeneratorOutputType,
        types::{ASTBlockType, UVType},
    },
    errors::SpannedError,
    tokens_parser::types::UVParseNode,
};

/// Parse Ultraviolet type
pub fn parse_type(node: &UVParseNode) -> GeneratorOutputType {
    Ok(ASTBlockType::Type(parse(node)?))
}

fn parse(node: &UVParseNode) -> Result<UVType, SpannedError> {
    if node.name.eq("union") {
        if node.self_closing {
            return Err(SpannedError::new(
                "Union cannot be used as individual type",
                node.span,
            ));
        }
        return parse_union(node);
    }

    if !node.self_closing {
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
        _ => {
            return Err(SpannedError::new(
                format!("Unknown type `{}`", node.name),
                node.span,
            ));
        }
    })
}

fn parse_union(node: &UVParseNode) -> Result<UVType, SpannedError> {
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
        let t = node.get_tag_at(0).ok_or(SpannedError::new(
            "[INTERNAL ERROR] Cannot get inner tag inside `union` block",
            node.span,
        ))?;

        return Ok(parse(t)?);
    }

    let types = node
        .get_all_tags()
        .into_iter()
        .map(parse)
        .collect::<Result<Vec<UVType>, SpannedError>>()?;

    Ok(UVType::new_union(types))
}
