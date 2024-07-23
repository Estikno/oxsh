use anyhow::Result;
use oxsh::{
    prompt,
    shell::{self, ShellStatus},
};

use rustyline::DefaultEditor;

fn main() -> Result<()> {
    let mut rl = DefaultEditor::new()?;

    loop {
        //read user input
        let input = prompt::read_input(&mut rl)?;

        //main logic
        match shell::shell_logic(&input)? {
            ShellStatus::Continue => continue,
            ShellStatus::Exit => break,
        }
    }

    Ok(())
}
