use anyhow::Result;

use oxsh::utils::get_history_path;
use oxsh::{
    prompt,
    shell::{self, ShellStatus},
};

use rustyline::Editor;

fn main() -> Result<()> {
    let rustyline_config = rustyline::config::Builder::new()
        .max_history_size(100)?
        .build();

    println!("Welcome to oxsh");
    println!("Type 'exit' to quit.");

    let path_history_file = get_history_path();
    let mut rl = Editor::with_config(rustyline_config)?;

    if rl.load_history(path_history_file.as_str()).is_err() {
        println!("No previous history file, creating a new one once the shell closes...")
    }

    loop {
        //read user input
        let user_input = prompt::read_input(&mut rl);

        if let Some(input) = user_input {
            //main logic
            match shell::shell_logic(&input, &mut rl)? {
                ShellStatus::Continue => continue,
                ShellStatus::Exit => break,
            }   
        }
    }

    rl.save_history(path_history_file.as_str())?;

    Ok(())
}
