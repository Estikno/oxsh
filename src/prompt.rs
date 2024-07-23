use anyhow::Result;
use std::io::{stdin, stdout, Write, BufRead};

//termion
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::cursor::{self, DetectCursorPos};

pub fn print_prompt() -> Result<()> {
    // use the `>` character as the prompt
    // need to explicitly flush this to ensure it prints before read_line
    print!("> ");
    stdout().flush()?;
    Ok(())
}

pub fn read_input() -> Result<String> {
    let mut input = String::new();
    
    //read input
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();
    let cursor_pos = stdout.cursor_pos()?;
    let mut index: usize = 0;
    
    write!(stdout, "{}", cursor::Hide)?;
    write!(stdout, "{}", cursor::Goto(0, cursor_pos.1))?;
    stdout.flush()?;

    for k in stdin.keys(){
        match k? {
            Key::Char('\n') => break,
            Key::Char(c) => {
                input.insert(index, c);
                index += 1;

                write!(stdout, "{}{}{}{}", termion::clear::CurrentLine, "> ", input, "▉")?;
                write!(stdout, "{}", cursor::Goto(0, cursor_pos.1))?;
            },
            Key::Backspace => {
                if index > 0 {
                    input.remove(index - 1);
                    index -= 1;

                    write!(stdout, "{}{}{}{}", termion::clear::CurrentLine, "> ", input, "▉")?;
                    write!(stdout, "{}", cursor::Goto(0, cursor_pos.1))?;
                }
            },
            _ => ()
        }
        stdout.flush()?;
    }

    write!(stdout, "{}{}", cursor::Show, termion::clear::CurrentLine)?;
    stdout.flush()?;

    Ok(input)
}
