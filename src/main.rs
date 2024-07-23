use anyhow::Result;

use oxsh::{
    prompt,
    shell::{self, ShellStatus},
};
use oxsh::utils::get_history_path;

use rustyline::DefaultEditor;

fn main() -> Result<()> {
    println!("Welcome to oxsh");
    println!("Type 'exit' to quit.");

    let path_history_file = get_history_path();

    let mut rl = DefaultEditor::new()?;

    if rl.load_history(path_history_file.as_str()).is_err() {
        println!("No previous history")
    }

    loop {
        //read user input
        let input = prompt::read_input(&mut rl)?;

        //main logic
        match shell::shell_logic(&input)? {
            ShellStatus::Continue => continue,
            ShellStatus::Exit => break,
        }
    }

    rl.save_history(path_history_file.as_str())?;

    Ok(())
}