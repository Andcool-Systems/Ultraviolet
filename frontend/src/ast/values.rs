use crate::{
    ast::types::{ASTBlockType, UVValue},
    errors::SpannedError,
    tokens_parser::types::UVParseNode,
};

/// Parse UVValues.
/// Caller must guarantee, that tag name is one of data types!
pub fn parse_value(node: UVParseNode) -> Result<ASTBlockType, SpannedError> {
    Ok(match node.name.as_str() {
        "int" => ASTBlockType::Value(UVValue::Int(parse_int(node)?)),
        "float" => ASTBlockType::Value(UVValue::Float(parse_float(node)?)),
        "str" => ASTBlockType::Value(UVValue::String(parse_str(node))),
        "bool" => ASTBlockType::Value(UVValue::Boolean(parse_boolean(node)?)),
        "null" => {
            validate_null(node)?;
            ASTBlockType::Value(UVValue::Null)
        }
        _ => unreachable!(),
    })
}

/// Guarantee, that node has only one child and this child is literal
fn validate_inner(node: &UVParseNode) -> Result<(), SpannedError> {
    if node.children_len() != 1 || !node.all_literals() {
        return Err(SpannedError::new(
            format!("Invalid value for `{}` type", node.name),
            node.span.clone(),
        ));
    }
    Ok(())
}

fn parse_int(node: UVParseNode) -> Result<i64, SpannedError> {
    validate_inner(&node)?;
    let inner_contents = node.get_inner_literal().unwrap(); // This unwrap is safe due checks above

    inner_contents.value.parse::<i64>().map_err(|_| {
        SpannedError::new(
            format!("Cannot parse `{}` to an integer", inner_contents.value),
            inner_contents.span.clone(),
        )
    })
}

fn parse_float(node: UVParseNode) -> Result<f64, SpannedError> {
    validate_inner(&node)?;
    let inner_contents = node.get_inner_literal().unwrap(); // This unwrap is safe due checks above

    inner_contents.value.parse::<f64>().map_err(|_| {
        SpannedError::new(
            format!("Cannot parse `{}` to an float", inner_contents.value),
            inner_contents.span.clone(),
        )
    })
}

fn parse_str(node: UVParseNode) -> String {
    let inner_contents = if let Some(lit) = node.get_inner_literal() {
        lit.value.clone()
    } else {
        String::new()
    };

    inner_contents
}

fn parse_boolean(node: UVParseNode) -> Result<bool, SpannedError> {
    validate_inner(&node)?;
    let inner_contents = node.get_inner_literal().unwrap(); // This unwrap is safe due checks above

    match inner_contents.value.as_str() {
        "1" | "true" => Ok(true),
        "0" | "false" => Ok(false),
        _ => Err(SpannedError::new(
            format!("Cannot parse `{}` to a boolean", inner_contents.value),
            inner_contents.span.clone(),
        )),
    }
}

fn validate_null(node: UVParseNode) -> Result<(), SpannedError> {
    if !node.self_closing {
        return Err(SpannedError::new(
            "`null` tag must be self-closing",
            node.span.clone(),
        ));
    }

    Ok(())
}
