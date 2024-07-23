use anyhow::Result;

use rustyline::Editor;

fn get_prompt() -> String {
    // use the `>` character as the prompt
    // need to explicitly flush this to ensure it prints before read_line
    String::from("> ")
}

pub fn read_input(rl: &mut Editor<(), rustyline::history::FileHistory>) -> Result<String> {
    match rl.readline(get_prompt().as_str()) {
        Ok(input) => {
            rl.add_history_entry(input.as_str())?;
            Ok(input)
        },
        Err(err) => {
            println!("Error: {:?}", err);
            Ok(String::new())
        }
    }
}
