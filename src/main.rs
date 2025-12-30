use anyhow::{Ok, Result};
use frontend;
use std::fs;

fn main() -> Result<()> {
    let code: String = fs::read_to_string("./examples/file.xll")?;
    frontend::get_ast(code);

    Ok(())
}
