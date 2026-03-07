use crate::{
    ast::GeneratorOutputType, errors::SpannedError, tokens_parser::types::UVParseNode,
    types::Positional,
};

pub fn parse_for_loop(node: &UVParseNode) -> GeneratorOutputType {
    let extra = node.search_extra_children(vec!["iterator", "start", "end", "step", "body"]);

    if !extra.is_empty() {
        let first_extra = extra.first().ok_or(SpannedError::new(
            "[INTERNAL ERROR] Cannot get inner extra tag",
            node.span,
        ))?;

        return Err(SpannedError::new(
            "Found extra children inside `for` loop declaration",
            first_extra.get_span(),
        ));
    }
    todo!()
}
