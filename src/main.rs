use std::io::{stdin, stdout, Write};
use std::process::{Child, Command};
use oxsh::commands::{self, cd};

fn main() {
    loop {
        // use the `>` character as the prompt
        // need to explicitly flush this to ensure it prints before read_line
        print!("> ");
        stdout().flush();

        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        // everything after the first whitespace character 
        //     is interpreted as args to the command
        let mut parts = input.trim().split_ascii_whitespace();
        let command = parts.next().unwrap();
        let args = parts;

        match command {
            "cd" => cd(args),
            "exit" => return,
            command => {
                let mut child = Command::new(command)
                    .args(args)
                    .spawn()
                    .unwrap();

                // don't accept another command until this one completes
                child.wait();
            }
        }
    }
}
