mod board;
mod field;
mod index;
mod player;
mod token;

use board::*;
use field::*;
use index::*;
use player::*;
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
                    if let Some(i) = Board::get_indices(x, y) {
                        board.play(i);

                        #[cfg(debug_assertions)]
                        write!(
                            stdout,
                            "{}{}  {:?}",
                            termion::cursor::Goto(30, 17 - board.modulo() as u16),
                            Fg(LightBlack),
                            i
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
