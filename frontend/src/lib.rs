use std::path::Path;

use crate::{
    ast::gen_main_ast, errors::error_renderer::ErrorRenderer, lexer::Lexer,
    tokens_parser::TokenParser, types::SourceFile,
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

    let parse = || {
        let mut lexer = Lexer::new(source.code.clone());
        let tokens = lexer.parse();

        let mut token_parser = TokenParser::new(tokens);
        let parse_tree = token_parser.parse()?;
        gen_main_ast(parse_tree)
    };

    match parse() {
        Ok(parse_tree) => println!("{:?}", parse_tree),
        Err(err) => eprintln!("{}", err.display_with_source(&source)),
    }

    Ok(())
}
