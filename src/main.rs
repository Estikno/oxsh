use std::io::{stdin, stdout, Write};
use std::process::{Child, Command, Stdio};
use oxsh::commands::cd;
use anyhow::Result;

fn main() -> Result<()> {
    'main_loop: loop {
        // use the `>` character as the prompt
        // need to explicitly flush this to ensure it prints before read_line
        print!("> ");
        stdout().flush()?;

        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        // must be peekable so we know when we are on the last command
        let mut commands = input.trim().split(" | ").peekable();
        let mut previous_command = None;

        while let Some(command) = commands.next() {
            // everything after the first whitespace character 
            //     is interpreted as args to the command
            let mut parts = command.trim().split_ascii_whitespace();
            let command = match parts.next() {
                Some(command) => command,
                None => { continue 'main_loop; },  // break if no command provided
            };
            let args = parts;

            match command {
                "cd" => {
                    cd(args);
                    previous_command = None;
                },
                "exit" => return Ok(()),
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
    }
}
