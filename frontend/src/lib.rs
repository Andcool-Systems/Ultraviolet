mod code_parser;
mod errors;
use code_parser::{CodeParser, types::ParseNode};

pub fn get_ast(code: String) -> ParseNode {
    let mut code_parser = CodeParser::new(code);
    println!("{:?}", code_parser.parse());
    todo!()
}
