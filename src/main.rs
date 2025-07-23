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

use termion::clear::All;
use termion::color::*;
use termion::cursor::*;
use termion::event::{Event, Key, MouseEvent};
use termion::input::{MouseTerminal, TermRead};
use termion::raw::IntoRawMode;
use termion::style::Reset;

use std::io::{Write, stdin, stdout};

#[cfg(debug_assertions)]
const TOKEN_COUNT: usize = 4;
#[cfg(not(debug_assertions))]
const TOKEN_COUNT: usize = 9;

fn main() {
    let board = &mut Board::init();

    let stdin = stdin();
    let mut stdout = MouseTerminal::from(stdout().into_raw_mode().unwrap());

    write!(stdout, "{}{}{board}", All, Goto(1, 1),).unwrap();
    stdout.flush().unwrap();

    for c in stdin.events() {
        write!(stdout, "{}{}{board}", All, Goto(1, 1),).unwrap();
        stdout.flush().unwrap();

        let evt = c.unwrap();
        match evt {
            Event::Key(Key::Ctrl('c')) => break,
            Event::Mouse(me) => match me {
                MouseEvent::Press(_, x @ 3..36, y @ 2..15) => {
                    if let Some(pos) = Position::translate(x, y) {
                        if let Some(winner) = board.play(&pos) {
                            writeln!(stdout, "{}{winner:?} won!\n\r", Fg(Red)).unwrap();
                            break;
                        }

                        #[cfg(debug_assertions)]
                        write!(stdout, "{}{}{pos:?}", Goto(2, 1), Fg(LightBlack),).unwrap();
                    };
                }
                _ => (),
            },
            _ => {}
        }
        #[cfg(debug_assertions)]
        stdout.flush().unwrap();
    }

    writeln!(stdout, "{}\n\r", Reset).unwrap();
    stdout.flush().unwrap();
}
