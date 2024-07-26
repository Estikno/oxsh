use rustyline::Editor;
use crate::utils::get_history_path;

/// Defines the possible arguments for the `history` command.
///
/// This enum represents the different actions that can be performed on the command history.
/// It includes options for clearing the history, rewriting it with the history file, writing it to the file,
/// and saving a command to the history without executing it.
/// Additionally, it includes an error handling variant for incorrect usage.
pub enum HistoryArgs {
    /// Clear the history in memory
    Clear,
    /// Rewrite the history in memory with the history file information
    Rewrite,
    /// Writes the history in memory to the history file
    Write,
    /// Saves a command in the history without excecuting it
    Save(String),
    /// Error handling args
    Error(String),
}

impl HistoryArgs {
    pub fn from_args(args: Vec<String>) -> HistoryArgs {
        match args.len() {
            0 => HistoryArgs::Error("Use: history [--clear | -c] [--rewrite | -r] [--write | -w] [--save | -s <command>]".to_string()),
            1 => match args[0].as_str() {
                "--clear" | "-c" => HistoryArgs::Clear,
                "--rewrite" | "-r" => HistoryArgs::Rewrite,
                "--write" | "-w" => HistoryArgs::Write,
                "--save" | "-s" => HistoryArgs::Error("Use: history [--clear | -c] [--rewrite | -r] [--write | -w] [--save | -s <command>]".to_string()),
                _ => HistoryArgs::Error("Use: history [--clear | -c] [--rewrite | -r] [--write | -w] [--save | -s <command>]".to_string()),
            },
            2 => match args[0].as_str() {
                "--save" | "-s" => HistoryArgs::Save(args[1].clone()),
                _ => HistoryArgs::Error("Use: history [--clear | -c] [--rewrite | -r] [--write | -w] [--save | -s <command>]".to_string()),
            },
            _ => HistoryArgs::Error("Too many arguments! Use: history [--clear | -c] [--rewrite | -r] [--write | -w] [--save | -s <command>]".to_string()),
        }
    }
}

pub fn history(args: HistoryArgs, editor: &mut Editor<(), rustyline::history::FileHistory>) {
    let path_history_file = get_history_path();
    let path_history_file = path_history_file.as_str();

    match args {
        HistoryArgs::Clear => {
            if editor.clear_history().is_err() {
                println!("There was an error clearing the history in memory!")
            }
        },
        HistoryArgs::Rewrite => {
            if editor.load_history(path_history_file).is_err() {
                println!("The history file does not exist! This might be because you tried to use the --rewrite option before closing the shell, which creates the history file.");
            }
        },
        HistoryArgs::Write => {
            if editor.save_history(path_history_file).is_err() {
                eprintln!("Failed to write to the history file!")
            }
        },
        HistoryArgs::Save(command) => {
            if editor.add_history_entry(command).is_err() {
                eprintln!("Failed to add command to the history!")
            }
        },
        HistoryArgs::Error(e) => println!("{}", e),
    }
}