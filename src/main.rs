use std::io::{stdin, stdout, Stdout, Write};
use std::process::{Command, Output};
use std::result::Result;

use termion::event::Key;
use termion::input::TermRead;
use termion::raw::{IntoRawMode, RawTerminal};

enum Notes {
    C, D, E, F, G, A, B
}

impl Notes {
    fn from_key(key: char) -> Result<Self, ()> {
        return match key {
            'a' => Ok(Notes::C),
            's' => Ok(Notes::D),
            'd' => Ok(Notes::E),
            'f' => Ok(Notes::F),
            'g' => Ok(Notes::G),
            'h' => Ok(Notes::A),
            'j' => Ok(Notes::B),
            _ => Err(())
        }
    }

    fn to_freq(&self) -> &'static str {
        return match self {
            Notes::C => "261",
            Notes::D => "293",
            Notes::E => "329",
            Notes::F => "349",
            Notes::G => "391",
            Notes::A => "440",
            Notes::B => "493"
        }
    }
}

fn beep(letter: char) -> Result<Output, ()> {
    let freq = Notes::from_key(letter)?.to_freq();
    return match Command::new("beep").arg("-f").arg(freq).output() {
        Ok(out) => Ok(out),
        Err(_) => Err(()) 
    };
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
            Key::Alt('t') => write!(stdout, "T-Rex\r\n").unwrap(),
            Key::Char(x) => match beep(x) {
                Err(_) => {
                    write!(stdout, "beep failed... exiting\r\n").unwrap();
                    break;
                }
                _ => (),
            },
            _ => ()
        }
        stdout.flush().unwrap();
    }
}
