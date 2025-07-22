mod board;
mod field;
mod gui;
mod player;
mod position;
mod token;

use board::*;
use field::*;
use player::*;
use position::*;
use token::*;

use termion::color::*;
use termion::cursor::Goto;
use termion::event::{Event, Key, MouseEvent};
use termion::input::{MouseTerminal, TermRead};
use termion::raw::IntoRawMode;
use termion::style::Reset;

use std::io::{Write, stdin, stdout};

fn main() {
    let board = &mut Board::init();

    let stdin = stdin();
    let mut stdout = MouseTerminal::from(stdout().into_raw_mode().unwrap());

    write!(stdout, "{}{}{board}", termion::clear::All, Goto(1, 1),).unwrap();
    stdout.flush().unwrap();

    for c in stdin.events() {
        write!(stdout, "{}{}{board}", termion::clear::All, Goto(1, 1),).unwrap();
        stdout.flush().unwrap();

        let evt = c.unwrap();
        match evt {
            Event::Key(Key::Ctrl('c')) => break,
            Event::Mouse(me) => match me {
                MouseEvent::Press(_, x @ 3..36, y @ 2..15) => {
                    if let Some(pos) = Position::translate(x, y) {
                        board.play(&pos);

                        #[cfg(debug_assertions)]
                        write!(
                            stdout,
                            "{}{}{}",
                            Goto(16, 8),
                            Fg(LightBlack),
                            format!(" {}", &pos)
                        )
                        .unwrap();
                    };
                }
                _ => (),
            },
            _ => {}
        }
        #[cfg(debug_assertions)]
        stdout.flush().unwrap();
    }

    // write!(stdout, "bye");
}
