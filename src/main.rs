use std::io::{stdin, stdout, Result, Stdout, Write};
use std::process::{Command, Output};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::{IntoRawMode, RawTerminal};

fn beep() -> Result<Output> {
    return Command::new("beep").arg("-f").arg("440").output();
}

fn clear_screen(toclear: &mut RawTerminal<Stdout>) {
    write!(
        toclear,
        "{}{}",
        termion::cursor::Goto(1, 1),
        termion::clear::All
    )
    .unwrap()
}

fn main() {
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();

    for c in stdin.keys() {
        clear_screen(&mut stdout);
        match c.unwrap() {
            Key::Ctrl('q') => break,
            Key::Ctrl('h') => write!(stdout, "Help!\r\n").unwrap(),
            Key::Alt('t') => write!(stdout, "T-Rex\r\n").unwrap(),
            _ => match beep() {
                Err(_) => {
                    write!(stdout, "beep failed... exiting\r\n").unwrap();
                    break;
                }
                _ => (),
            },
        }
        stdout.flush().unwrap();
    }
}
