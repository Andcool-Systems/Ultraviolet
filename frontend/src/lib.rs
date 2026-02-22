use crate::lexer::Lexer;

mod errors;
mod iterator;
mod lexer;

pub fn get_ast(code: String) {
    let mut lexer = Lexer::new(code.clone());
    let tokens = lexer.parse();
    println!("{:?}", tokens);
    println!("newline indexes: {:?}", lexer.get_lines_indexes());
}
