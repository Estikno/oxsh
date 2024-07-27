//crates
use dirs;
use rustyline::{Editor, history::FileHistory};

use std::process::Command;

//static variables
static HISTORY_FILE: &str = ".oxsh_history";
static MAX_HISTORY_SIZE: usize = 100;
static INIT_FILE: &str = "oxshrc.sh";


/// Retrieves the path to the history file.
///
/// # Returns
///
/// The path to the history file as a `String`.
pub fn get_history_path() -> String {
    // Get the home directory of the user.
    let home_dir = dirs::home_dir().expect("Failed to get home directory");

    // Join the home directory with the history file name to get the full path.
    let history_file_path = home_dir.join(HISTORY_FILE);

    // Convert the path to a string and return it.
    history_file_path.to_str().unwrap().to_string()
}

pub fn initialize_rustyline_editor(path_history_file: &String) -> Result<Editor<(), FileHistory>, rustyline::error::ReadlineError> {
    //configures rustyline
    let rustyline_config = rustyline::config::Builder::new()
        .max_history_size(MAX_HISTORY_SIZE)?
        .build();

    let mut rl = Editor::with_config(rustyline_config)?;
    
    //load the history
    rl.load_history(path_history_file.as_str()).unwrap_or_else(|_| println!("No previous history file, creating a new one once the shell closes..."));

    return Ok(rl);
}

pub fn read_init_script() {
    // Get the home directory of the user.
    let home_dir = dirs::home_dir().expect("Failed to get home directory");

    // Join the home directory with the history file name to get the full path.
    let init_file_path = home_dir.join(INIT_FILE);

    let command = Command::new("bash")
        .arg(init_file_path)
        .spawn();

    if let Ok(mut command) = command {
        command.wait().unwrap();
    }
    else {
        println!("Failed to run init script!");
    }
}