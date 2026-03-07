use anyhow::Result;
use regex::Regex;

use crate::{
    ast::{
        compare_op::parse_compare_op,
        logical_op::parse_logical_op,
        loops::parse_for_loop,
        math_op::parse_math_op,
        traits::{StringToUVCompareOp, StringToUVLogicalOp, StringToUVMathOp, StringToUVType},
        type_parser::parse_type,
        types::{ASTBlockType, ProgramBlock},
        values::parse_value,
        variables::{parse_var_access, parse_var_assign, parse_var_definition},
    },
    errors::SpannedError,
    tokens_parser::types::UVParseNode,
};
use once_cell::sync::Lazy;

mod compare_op;
mod logical_op;
mod loops;
mod math_op;
mod traits;
mod type_parser;
mod types;
mod values;
mod variables;

pub type GeneratorOutputType = Result<ASTBlockType, SpannedError>;

static IDENT_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"^[a-zA-Z_][a-zA-Z0-9_]*$").unwrap());

/// Check if provided string is a valid var/fn identifier
fn is_valid_identifier(s: &str) -> bool {
    IDENT_REGEX.is_match(s)
}

/// Parse <program> content
pub fn gen_main_ast(node: &UVParseNode) -> GeneratorOutputType {
    if node.name.ne("program") {
        return Err(SpannedError::new(
            "The program must begin with the <program> tag",
            node.span,
        ));
    }

    let head_parsed = if let Some(h) = node.get_child_by_name("head") {
        Some(ASTBlockType::HeadBlock(parse_root_children(&h)?))
    } else {
        None
    };

    let main =
        ASTBlockType::MainBlock(parse_root_children(node.get_child_by_name("main").ok_or(
            SpannedError::new("Main block in <program> is required", node.span),
        )?)?);

    Ok(ASTBlockType::Program(Box::new(ProgramBlock {
        head: head_parsed,
        main: main,
        span: node.span,
    })))
}

/// Main recursively invoked parsing function
pub fn generate_ast(node: &UVParseNode) -> GeneratorOutputType {
    Ok(match node.name.as_str() {
        // Parse variable declaration
        "let" if !node.self_closing => parse_var_definition(node)?,

        // Parse for loop declaration
        "for" if !node.self_closing => parse_for_loop(node)?,

        // Type parsing
        // FIXME: Parsing of types should only occur in special places
        // TODO: Move this parsing to a separate function
        name if name.to_uvtype().is_some() && node.self_closing => parse_type(node)?,
        "union" if !node.self_closing => parse_type(node)?,

        // Values such as int, float, etc.
        name if name.to_uvtype().is_some() => parse_value(node)?,

        // Parse math operations, such as sum, div, etc.
        name if name.to_uvmath().is_some() && !node.self_closing => parse_math_op(node)?,

        // Parse compare operators, such as eq, neq, etc.
        name if name.to_uvcompare().is_some() && !node.self_closing => parse_compare_op(node)?,

        // Parse logical operators, such as and, or, not
        name if name.to_uvlogical().is_some() && !node.self_closing => parse_logical_op(node)?,

        // Parse variable assign
        _ if !node.self_closing => parse_var_assign(node)?,

        // Parse variable access
        _ if node.self_closing => parse_var_access(node)?,

        name => {
            return Err(SpannedError::new(
                format!("Unexpected `{name}` tag"),
                node.span,
            ));
        }
    })
}

/// Parse children in head and main tags
fn parse_root_children(node: &UVParseNode) -> Result<Vec<ASTBlockType>, SpannedError> {
    if !node.all_tags() {
        let first_literal = node.get_inner_literal().ok_or(SpannedError::new(
            "[INTERNAL ERROR] Cannot get inner literal for error",
            node.span,
        ))?;

        return Err(SpannedError::new(
            "Unexpected unwrapped literal in root tag",
            first_literal.span,
        ));
    }

    node.get_all_tags()
        .iter()
        .map(|ch| Ok(generate_ast(ch)?))
        .collect::<Result<Vec<ASTBlockType>, SpannedError>>()
}
