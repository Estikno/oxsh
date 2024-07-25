use crate::commands::{cd, help};
use anyhow::Result;
use std::{process::{Child, Command, Stdio}, str::SplitAsciiWhitespace};

pub enum ShellStatus {
    Continue,
    Exit,
}

pub fn shell_logic(input: &String) -> Result<ShellStatus> {
    // must be peekable so we know when we are on the last command
    let mut commands = input.trim().split(" | ").peekable();
    let mut previous_command = None;

    while let Some(command) = commands.next() {
        // everything after the first whitespace character
        //     is interpreted as args to the command
        let (command, args) = match parse_command(command) {
            Some((command, args)) => (command, args),
            None => return Ok(ShellStatus::Continue)
        };

        match command {
            "cd" => {
                cd(args);
                previous_command = None;
            }
            "help" => help(),
            "exit" => return Ok(ShellStatus::Exit),
            command => {
                let stdin = previous_command.map_or(Stdio::inherit(), |output: Child| {
                    Stdio::from(output.stdout.unwrap())
                });

                let stdout = if commands.peek().is_some() {
                    // there is another command piped behind this one
                    // prepare to send output to the next command
                    Stdio::piped()
                } else {
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
                    Ok(output) => {
                        previous_command = Some(output);
                    }
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

/// Parses a command from a given input string.
///
/// # Arguments
///
/// * `input` - The input string to parse the command from.
///
/// # Returns
///
/// * `Some((command, args))` - A tuple containing the command and the arguments.
/// * `None` - If no command is provided in the input string.
fn parse_command(input: &str) -> Option<(&str, SplitAsciiWhitespace)> {
    // Split the input string into parts separated by whitespace characters.
    let mut parts = input.trim().split_ascii_whitespace();

    // Get the first part as the command.
    let command = match parts.next() {
        Some(command) => command,
        None => {
            return None; // Return None if no command is provided.
        }
    };

    // Collect the remaining parts as the arguments.
    let args: SplitAsciiWhitespace = parts;

    // Return the command and arguments as a tuple.
    Some((command, args))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_command_with_command_and_args() {
        let input = "ls -l";
        let expected_command = "ls";
        let expected_args: Vec<&str> = vec!["-l"];

        let result = parse_command(input).unwrap();

        assert_eq!(result.0, expected_command);
        assert_eq!(result.1.collect::<Vec<&str>>(), expected_args);
    }

    #[test]
    fn test_parse_command_with_command_only() {
        let input = "ls";
        let expected_command = "ls";
        let expected_args: Vec<&str> = vec![];

        let result = parse_command(input).unwrap();
        
        assert_eq!(result.0, expected_command);
        assert_eq!(result.1.collect::<Vec<&str>>(), expected_args);
    }

    #[test]
    fn test_parse_command_with_no_command() {
        let input = "";
        assert_eq!(parse_command(input).is_none(), true);
    }
}