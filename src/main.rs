use anyhow::{Ok, Result};
use frontend;
use std::path::Path;

fn main() -> Result<()> {
    let _ = frontend::process(Path::new("./examples/file.uv"));

    Ok(())
}
