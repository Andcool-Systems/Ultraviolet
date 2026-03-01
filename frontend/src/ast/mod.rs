use anyhow::Result;
use colored::Colorize;
use regex::Regex;

use crate::{
    ast::{
        traits::StringToUVType,
        types::{ASTBlockType, ProgramBlock, VariableDefinition},
        values::parse_value,
    },
    errors::SpannedError,
    tokens_parser::types::{UVParseBody, UVParseNode},
    types::TypeWithSpan,
};

mod traits;
mod types;
mod values;

type GeneratorOutputType = Result<ASTBlockType, SpannedError>;

/// Check if provided string is a valid var/fn identifier
fn is_valid_identifier(s: &str) -> bool {
    Regex::new(r"^[a-zA-Z_][a-zA-Z0-9_]*$").unwrap().is_match(s)
}

pub fn gen_main_ast(parse_tree: UVParseNode) -> GeneratorOutputType {
    if parse_tree.name.ne("program") {
        return Err(SpannedError::new(
            "The program must begin with the <program> tag",
            parse_tree.span.clone(),
        ));
    }

    Ok(parse_program_block(parse_tree)?)
}

pub fn generate_ast(parse_tree: UVParseNode) -> GeneratorOutputType {
    Ok(match parse_tree.name.as_str() {
        "let" => parse_var_definition(parse_tree)?,

        // Values such as int, float, etc.
        name if name.to_owned().to_uvtype().is_some() => parse_value(parse_tree)?,

        name => {
            return Err(SpannedError::new(
                format!("Unexpected <{name}> tag"),
                parse_tree.span.clone(),
            ));
        }
    })
}

/// Parse <program> content
fn parse_program_block(node: UVParseNode) -> GeneratorOutputType {
    let head = node.get_child_by_name("head");

    let head_parsed = if let Some(h) = head {
        Some(ASTBlockType::HeadBlock(parse_root_children(
            h.children.clone(),
        )?))
    } else {
        None
    };

    let main = ASTBlockType::MainBlock(parse_root_children(
        node.get_child_by_name("main")
            .ok_or(SpannedError::new(
                "Main block in <program> is required",
                node.span.clone(),
            ))?
            .children
            .clone(),
    )?);

    Ok(ASTBlockType::Program(Box::new(ProgramBlock {
        head: head_parsed,
        main: main,
        span: node.span,
    })))
}

/// Parse children in head and main tags
fn parse_root_children(children: Vec<UVParseBody>) -> Result<Vec<ASTBlockType>, SpannedError> {
    children
        .into_iter()
        .map(|ch| match ch {
            UVParseBody::String(uvparse_literal) => {
                return Err(SpannedError::new(
                    "Unexpected unwrapped literal in root tag",
                    uvparse_literal.span.clone(),
                ));
            }
            UVParseBody::Tag(uvparse_node) => Ok(generate_ast(*uvparse_node)?),
        })
        .collect::<Result<Vec<ASTBlockType>, SpannedError>>()
}

/// Parse definition of variables <let>
fn parse_var_definition(node: UVParseNode) -> GeneratorOutputType {
    let name_block = node.get_child_by_name("name").ok_or(SpannedError::new(
        "Variable definition should have an inner <name> tag",
        node.span.clone(),
    ))?;

    if name_block.children_len() != 1 || !name_block.all_literals() {
        return Err(SpannedError::new(
            "Invalid variable name",
            name_block.span.clone(),
        ));
    }

    let name = name_block.get_inner_literal().unwrap(); // This unwrap is unreachable due checks above

    if !is_valid_identifier(&name.value) {
        return Err(SpannedError::new(
            format!("`{}` is not a valid name for variable", name.value),
            name.span.clone(),
        ));
    }

    let value_block = node.get_child_by_name("value").ok_or(SpannedError::new(
        "Variable must be initialized",
        node.span.clone(),
    ))?;

    if value_block.children_len() != 1 || !value_block.all_tags() {
        return Err(SpannedError::new(
            format!(
                "Variable value must have only one inner tag.\n{}{}",
                "tip".green(),
                ": If you want to place multiple tags, wrap them in a <b> tag.",
            ),
            value_block.span.clone(),
        ));
    }

    let value = value_block.get_child_node(0).unwrap(); // This unwrap is unreachable due checks above

    let is_const = if let Some(c) = node.get_child_by_name("const") {
        c.self_closing
    } else {
        false
    };

    Ok(ASTBlockType::VariableDefinition(VariableDefinition {
        name: TypeWithSpan::new(name.value.clone(), name_block.span.clone()),
        value: TypeWithSpan::new(
            Box::new(generate_ast(value.clone())?),
            value_block.span.clone(),
        ),
        is_const: is_const,

        span: node.span,
    }))
}
