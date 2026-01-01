pub mod parser_error;
pub mod simple;

#[allow(dead_code)]
pub enum ErrorKind {
    Parsing,
    DefinitionCheck,
    TypeCheck,
    MathProcessing,
}
