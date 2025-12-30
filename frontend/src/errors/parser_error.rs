use colored::Colorize;
use std::process;

use crate::code_parser::iterator::Iter;

pub struct ParserError {}
const T_SYMBOL: char = '┬';
const L_SYMBOL: char = '└';
const J_SYMBOL: char = '┘';
const D_SYMBOL: char = '─';

impl ParserError {
    fn trimmed_chars(s: &str) -> usize {
        s.len() - s.trim_start().len()
    }

    pub fn error(mess: &str, iter: &mut Iter<char>) -> ! {
        let mut error_char: usize = iter.pos - 1;
        let mut buf = String::new();
        while let Some(c) = iter.step_back() {
            if c == '\n' {
                break;
            }
        }
        let iter_pos = iter.pos;
        if iter_pos != 0 {
            error_char -= iter.pos + 1;
            iter.next();
        }

        while let Some(c) = iter.next() {
            if c == '\n' {
                break;
            }
            buf.push(c);
        }

        let _error_char = error_char - Self::trimmed_chars(&buf);
        buf = buf.trim().to_owned();
        let (left, char, right) = (
            &buf[.._error_char],
            &buf[_error_char.._error_char + 1],
            &buf[_error_char + 1..],
        );

        let error_str = if _error_char >= mess.len() + 2 {
            Self::compile_left(mess, _error_char)
        } else {
            Self::compile_right(mess, _error_char)
        };

        eprintln!("{}{}{}\n{}", left, char.red(), right, error_str,);
        process::exit(-1);
    }

    fn compile_left(msg: &str, char_pos: usize) -> String {
        let offset = char_pos - (msg.len() + 2);
        format!(
            "{}{}\n{}{} {}{}",
            " ".repeat(char_pos),
            T_SYMBOL,
            " ".repeat(offset),
            msg.red().bold(),
            D_SYMBOL,
            J_SYMBOL
        )
    }

    fn compile_right(msg: &str, char_pos: usize) -> String {
        let offset = char_pos;
        format!(
            "{}{}\n{}{}{} {}",
            " ".repeat(char_pos),
            T_SYMBOL,
            " ".repeat(offset),
            L_SYMBOL,
            D_SYMBOL,
            msg.red().bold(),
        )
    }
}
