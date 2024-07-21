use anyhow::Result;
use std::io;
use oxsh::{
    prompt,
    shell::{self, ShellStatus},
};

fn main() -> Result<()> {
    loop {
        //print prompt, the graphical representation
        prompt::print_prompt()?;

        //read user input
        let input = prompt::read_input()?;

        //main logic
        match shell::shell_logic(&input)? {
            ShellStatus::Continue => continue,
            ShellStatus::Exit => break,
        }
    }

    Ok(())
}
