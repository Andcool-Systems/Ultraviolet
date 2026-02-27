use std::path::Path;

use crate::{
    errors::error_renderer::ErrorRenderer, lexer::Lexer, tokens_parser::TokenParser,
    types::SourceFile,
};
use anyhow::Result;

mod ast;
mod errors;
mod iterator;
mod lexer;
mod tokens_parser;
mod types;

pub fn process(file_path: &Path) -> Result<()> {
    let source = SourceFile::load(file_path)?;

    let mut lexer = Lexer::new(source.code.clone());
    let tokens = lexer.parse();

    let mut token_parser = TokenParser::new(tokens);
    match token_parser.parse() {
        Ok(parse_tree) => println!("{:?}", parse_tree),
        Err(err) => eprintln!("{}", err.display_with_source(&source)),
    }
    Ok(())
}
