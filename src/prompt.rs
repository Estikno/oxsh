use anyhow::Result;
use std::io::{stdout, Write, BufRead};

pub fn print_prompt() -> Result<()> {
    // use the `>` character as the prompt
    // need to explicitly flush this to ensure it prints before read_line
    print!("> ");
    stdout().flush()?;
    Ok(())
}

pub fn read_input(reader: &mut dyn BufRead) -> Result<String> {
    let mut input = String::new();
    reader.read_line(&mut input)?;
    Ok(input)
}
