use anyhow::Result;

use rustyline::Editor;

fn get_prompt() -> String {
    // use the `>` character as the prompt
    // need to explicitly flush this to ensure it prints before read_line
    String::from("> ")
}


/// Reads user input from the console using the rustyline library
///
/// # Arguments
///
/// * `rl` - A mutable reference to the rustyline Editor object
///
/// # Returns
///
/// * `Result<String, anyhow::Error>` - The user input as a String,
///   or an error if there was a problem reading the input.
pub fn read_input(rl: &mut Editor<(), rustyline::history::FileHistory>) -> Result<String> {
    // Read user input from the console using the rustyline library
    match rl.readline(get_prompt().as_str()) {
        // If the input was successfully read, add it to the history and return it
        Ok(input) => {
            rl.add_history_entry(input.as_str())?;
            Ok(input)
        },
        // If there was an error reading the input, print the error and return an empty String
        Err(err) => {
            println!("Error: {:?}", err);
            Ok(String::new())
        }
    }
}
