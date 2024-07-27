//Our mods
pub mod commands;
pub mod prompt;
pub mod shell;
pub mod utils;

use crate::utils::get_history_path;
use crate::shell::ShellStatus;

use anyhow::Result;

fn main() -> Result<()> {
    let path_history_file = get_history_path();
    let mut rl = utils::initialize_rustyline_editor(&path_history_file)?;

    utils::read_init_script();

    //REPL LOOP
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
