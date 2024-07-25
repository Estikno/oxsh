use crate::commands::{cd, help, CommandType};
use anyhow::Result;
use std::{
    process::{Child, Command, Stdio},
    str::SplitAsciiWhitespace,
};

pub enum ShellStatus {
    Continue,
    Exit,
}

/// Executes a series of commands with pipes between them.
///
/// # Arguments
///
/// * `input` - A string containing the commands to be executed
///
/// # Returns
///
/// * `Result<ShellStatus>` - The status of the shell after executing the commands.
pub fn shell_logic(input: &String) -> Result<ShellStatus> {
    // must be peekable so we know when we are on the last command
    let mut commands = input.trim().split(" | ").peekable();
    let mut previous_command = None;
    while let Some(command) = commands.next() {
        // everything after the first whitespace character
        //     is interpreted as args to the command
        let (command, args) = match parse_command(command) {
            Some((command, args)) => (command, args),
            None => return Ok(ShellStatus::Continue), // empty command
        };
        let com_type = CommandType::from_str(command);
        let result = execute_command(
            com_type,
            args,
            &mut previous_command,
            commands.peek().is_some(), // are there more commands to execute?
        );
        if let ShellStatus::Exit = result {
            return Ok(ShellStatus::Exit);
        }
    }

    if let Some(mut final_command) = previous_command {
        // block until the final command has finished
        final_command.wait()?;
    }

    Ok(ShellStatus::Continue)
}


/// Executes a command based on its type.
///
/// # Arguments
///
/// * `com_type` - The type of the command.
/// * `args` - The arguments passed to the command.
/// * `previous_command` - A mutable reference to the previous command.
/// * `has_next` - A boolean indicating if there are more commands to execute.
///
/// # Returns
///
/// * `ShellStatus::Continue` - If the command is executed successfully.
/// * `ShellStatus::Exit` - If the command is the exit command.
fn execute_command(
    com_type: CommandType,
    args: SplitAsciiWhitespace,
    previous_command: &mut Option<Child>,
    has_next: bool,
) -> ShellStatus {
    // Execute the command based on its type.
    match com_type {
        CommandType::CD => {
            // Execute the 'cd' command.
            cd(args);
            *previous_command = None;
        }
        CommandType::Help => {
            // Execute the 'help' command.
            help();
        }
        CommandType::Exit => {
            // If the command is the exit command, return ShellStatus::Exit.
            return ShellStatus::Exit;
        }
        CommandType::External(command) => {
            // Execute an external command.
            let stdin = previous_command
                .take()
                .map_or(Stdio::inherit(), |output: Child| {
                    // Set the stdin of the new command to the output of the previous command.
                    Stdio::from(output.stdout.unwrap())
                });

            let stdout = if has_next {
                // If there are more commands to execute, set the stdout to a piped stream.
                Stdio::piped()
            } else {
                // Otherwise, set the stdout to the inherited stream.
                Stdio::inherit()
            };

            let output = Command::new(command)
                .args(args)
                .stdin(stdin)
                .stdout(stdout)
                .spawn();

            match output {
                Ok(output) => {
                    // Set the previous command to the new output.
                    *previous_command = Some(output);
                }
                Err(e) => {
                    // If there is an error, print the error and reset the previous command.
                    *previous_command = None;
                    eprintln!("{}", e);
                }
            };
        }
    }

    // Return ShellStatus::Continue if the command is executed successfully.
    ShellStatus::Continue
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

        assert!(parse_command(input).is_none())
    }
}
