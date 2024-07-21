use std::{env, path::Path};

pub fn cd(args: std::str::SplitAsciiWhitespace){
    if args.clone().count() > 1 {
        eprintln!("cd: too many arguments");
        return;
    }

    // default to '/' as new directory if one was not provided
    let new_dir = args.peekable().peek().map_or("/", |x| *x);
    let root = Path::new(new_dir);

    if let Err(e) = env::set_current_dir(&root){
        eprintln!("{}", e);
    }
}

pub fn help(){
    println!("                              _____  ______  _   _");
    println!("                             / _ \\ \\/ / ___|| | | |");
    println!("                            | | | \\  /\\___ \\| |_| |");
    println!("                            | |_| /  \\ ___) |  _  |");
    println!("                             \\___/_/\\_\\____/|_| |_|");
    println!("                                                   ");
    println!("oxsh - a minimalist shell");
    println!("Built-in commands:");
    println!("cd <directory>: Change the current directory");
    println!("help: Display this help message");
    println!("exit: Exit the shell");
}