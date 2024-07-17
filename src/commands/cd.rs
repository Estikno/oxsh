use std::{env, path::Path};

pub fn cd(args: std::str::SplitAsciiWhitespace){
    // default to '/' as new directory if one was not provided
    let new_dir = args.peekable().peek().map_or("/", |x| *x);
    let root = Path::new(new_dir);

    if let Err(e) = env::set_current_dir(&root){
        eprintln!("{}", e);
    }
}