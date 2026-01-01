use super::ErrorKind;
use colored::Colorize;
use std::process;

#[allow(dead_code)]
pub struct SimpleError {}

impl SimpleError {
    #[allow(dead_code)]
    fn get_error_kind(error_kind: ErrorKind) -> String {
        match error_kind {
            ErrorKind::Parsing => String::from("Parsing error"),
            ErrorKind::DefinitionCheck => String::from("Definition check error"),
            ErrorKind::TypeCheck => String::from("Type check error"),
            ErrorKind::MathProcessing => String::from("Math processing error"),
        }
    }

    #[allow(dead_code)]
    pub fn error(mess: &str, error_kind: ErrorKind) -> ! {
        eprintln!(
            "\n{}: {}",
            Self::get_error_kind(error_kind).red().bold(),
            mess
        );
        process::exit(-1);
    }
}
