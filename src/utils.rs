use dirs;

static HISTORY_FILE: &str = ".oxsh_history";


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
