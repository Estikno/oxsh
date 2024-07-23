use std::path::PathBuf;
use dirs;

static HISTORY_FILE: &str = ".oxsh_history";

pub fn get_history_path() -> String {
    let home_dir = dirs::home_dir().expect("Can't find home directory");
    let history_path: PathBuf = [home_dir.to_str().unwrap(), HISTORY_FILE].iter().collect();
    history_path.to_str().unwrap().to_string()
}