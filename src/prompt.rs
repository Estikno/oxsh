use std::io::{stdin, stdout, Write};
use anyhow::Result;

pub fn print_prompt() -> Result<()> {
    // use the `>` character as the prompt
    // need to explicitly flush this to ensure it prints before read_line
    print!("> ");
    stdout().flush()?;
    Ok(())
}

pub fn read_input() -> Result<String> {
    let mut input = String::new();
    stdin().read_line(&mut input)?;
    Ok(input)
}