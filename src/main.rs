use std::io::{stdin, stdout, Write};
use std::process::Command;
use oxsh::commands::cd;
use anyhow::Result;

fn main() -> Result<()> {
    loop {
        // use the `>` character as the prompt
        // need to explicitly flush this to ensure it prints before read_line
        print!("> ");
        stdout().flush()?;

        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        // everything after the first whitespace character 
        //     is interpreted as args to the command
        let mut parts = input.trim().split_ascii_whitespace();
        let command;

        match parts.next() {
            Some(com) => command = com,
            None => continue,  // continue if no command provided
        }

        let args = parts;

        match command {
            "cd" => cd(args),
            "exit" => break,
            command => {
                let child = Command::new(command)
                    .args(args)
                    .spawn();

                match child {
                    Ok(mut child) => { child.wait()?; },
                    Err(e) => eprintln!("{}", e)
                };
            }
        }
    }

    Ok(())
}
