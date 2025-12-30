pub mod parser_error;
pub mod simple;

pub enum ErrorKind {
    Parsing,
    DefinitionCheck,
    TypeCheck,
    MathProcessing,
}
