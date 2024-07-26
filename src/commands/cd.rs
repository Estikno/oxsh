use std::{env, path::Path};

/// Changes the current working directory.
///
/// # Arguments
///
/// * `args` - An iterator over the arguments passed to the `cd` command.
///            The function expects at most one argument, which is the directory to change to.
///            If no argument is provided, the function defaults to the root directory.
pub fn cd(args: Vec<String>) {
    // Check if the number of arguments is greater than one
    if args.len() > 1 {
        // If there are too many arguments, print an error message and return
        eprintln!("cd: too many arguments");
        return;
    }

    // Use peekable() to get an iterator over the arguments
    // Use peek() to get the first argument (if any)
    // Use map_or() to provide a default value ("/") if no argument is provided
    let new_dir = args.iter().peekable().peek().map_or("/", |x| *x);
    // Create a Path object from the new directory
    let root = Path::new(new_dir);

    // Try to change the current working directory to the new directory
    // If there is an error, print the error message
    if let Err(e) = env::set_current_dir(&root) {
        eprintln!("{}", e);
    }
}