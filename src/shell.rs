use std::process::{Child, Command, Stdio};
use anyhow::Result;
use crate::commands::cd;

pub enum ShellStatus {
    Continue,
    Exit
}

pub fn shell_logic(input: &String) -> Result<ShellStatus> {
    // must be peekable so we know when we are on the last command
    let mut commands = input.trim().split(" | ").peekable();
    let mut previous_command = None;

    while let Some(command) = commands.next() {
        // everything after the first whitespace character 
        //     is interpreted as args to the command
        let mut parts = command.trim().split_ascii_whitespace();
        let command = match parts.next() {
            Some(command) => command,
            None => { return Ok(ShellStatus::Continue); },  // breaks if no command provided
        };
        let args = parts;

        match command {
            "cd" => {
                cd(args);
                previous_command = None;
            },
            "exit" => return Ok(ShellStatus::Exit),
            command => {
                let stdin = previous_command
                    .map_or(
                        Stdio::inherit(),
                        |output: Child| Stdio::from(output.stdout.unwrap())
                    );
                
                let stdout = if commands.peek().is_some() {
                    // there is another command piped behind this one
                    // prepare to send output to the next command
                    Stdio::piped()
                }
                else{
                    // there are no more commands piped behind this one
                    // send output to shell stdout
                    Stdio::inherit()
                };

                let output = Command::new(command)
                    .args(args)
                    .stdin(stdin)
                    .stdout(stdout)
                    .spawn();

                match output {
                    Ok(output) => { previous_command = Some(output); },
                    Err(e) => {
                        previous_command = None;
                        eprintln!("{}", e);
                    }
                };
            }
        }
    }

    if let Some(mut final_command) = previous_command {
        // block until the final command has finished
        final_command.wait()?;
    }

    Ok(ShellStatus::Continue)
}