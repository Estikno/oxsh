use std::{env, path::Path};

// TODO: Add arguments to each command that needs it. Lifetimes are needed for args.
pub enum CommandType {
    CD,
    Help,
    Exit,
    External(String),
}

impl CommandType {
    pub fn from_str(command: &str) -> CommandType {
        match command {
            "cd" => CommandType::CD,
            "help" => CommandType::Help,
            "exit" => CommandType::Exit,
            com => CommandType::External(com.to_string()),
        }
    }
}

/// Changes the current working directory.
///
/// # Arguments
///
/// * `args` - An iterator over the arguments passed to the `cd` command.
///            The function expects at most one argument, which is the directory to change to.
///            If no argument is provided, the function defaults to the root directory.
pub fn cd(args: std::str::SplitAsciiWhitespace) {
    // Check if the number of arguments is greater than one
    if args.clone().count() > 1 {
        // If there are too many arguments, print an error message and return
        eprintln!("cd: too many arguments");
        return;
    }

    // Use peekable() to get an iterator over the arguments
    // Use peek() to get the first argument (if any)
    // Use map_or() to provide a default value ("/") if no argument is provided
    let new_dir = args.peekable().peek().map_or("/", |x| *x);
    // Create a Path object from the new directory
    let root = Path::new(new_dir);

    // Try to change the current working directory to the new directory
    // If there is an error, print the error message
    if let Err(e) = env::set_current_dir(&root) {
        eprintln!("{}", e);
    }
}

pub fn help() {
    println!("                              _____  ______  _   _");
    println!("                             / _ \\ \\/ / ___|| | | |");
    println!("                            | | | \\  /\\___ \\| |_| |");
    println!("                            | |_| /  \\ ___) |  _  |");
    println!("                             \\___/_/\\_\\____/|_| |_|");
    println!("                                                   ");
    println!("oxsh - a minimalist shell");
    println!("");
    println!("Built-in commands:");
    println!("cd <directory>: Change the current directory");
    println!("help: Display this help message");
    println!("exit: Exit the shell");
}
