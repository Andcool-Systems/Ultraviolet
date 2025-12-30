mod code_parser;
mod errors;
use code_parser::{CodeParser, types::ASTNode};

pub fn get_ast(code: String) -> ASTNode {
    let mut code_parser = CodeParser::new(code);
    println!("{:?}", code_parser.parse());
    todo!()
}
