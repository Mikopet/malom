#![allow(unused)]
use board::Board;
use field::Field;
use index::Index;
use player::Player;
use std::io::{Write, stdin, stdout};
use termion::event::{Event, Key, MouseEvent};
use termion::input::{MouseTerminal, TermRead};
use termion::raw::IntoRawMode;

use std::{collections::HashMap, fmt::Display};

mod board;
mod field;
mod index;
mod player;

fn main() {
    let mut board = Board::init();

    let stdin = stdin();
    let mut stdout = MouseTerminal::from(stdout().into_raw_mode().unwrap());

    write!(
        stdout,
        "{}{}{board}",
        termion::clear::All,
        termion::cursor::Goto(1, 1),
    )
    .unwrap();
    stdout.flush().unwrap();

    for c in stdin.events() {
        write!(
            stdout,
            "{}{}{board}",
            termion::clear::All,
            termion::cursor::Goto(1, 1),
        );
        let evt = c.unwrap();
        match evt {
            Event::Key(Key::Char('q')) => break,
            Event::Mouse(me) => match me {
                MouseEvent::Press(_, x @ 3..36, y @ 2..15) => {
                    let index = Board::get_by_tui(x, y);
                    if let Some(v) = board.get(index) {
                        // if let Field::Valid(_p) = v {
                        match index {
                            (Index::Invalid, _) => {}
                            (_, Index::Invalid) => {}
                            _ => {
                                board.play(index);
                            }
                        }
                        // }
                    }

                    #[cfg(debug_assertions)]
                    write!(stdout, "{}|{:?}|", termion::cursor::Goto(30, 16), index).unwrap();
                }
                _ => (),
            },
            _ => {}
        }
        stdout.flush().unwrap();
    }

    // write!(stdout, "bye");
}
