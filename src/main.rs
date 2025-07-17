use std::io::{Write, stdin, stdout};
use termion::event::{Event, Key, MouseEvent};
use termion::input::{MouseTerminal, TermRead};
use termion::raw::IntoRawMode;

mod board;
mod field;
mod index;
mod player;

use board::Board;

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
        )
        .unwrap();
        let evt = c.unwrap();
        match evt {
            Event::Key(Key::Ctrl('c')) => break,
            Event::Mouse(me) => match me {
                MouseEvent::Press(_, x @ 3..36, y @ 2..15) => {
                    if let Some(i) = Board::get_indices(x, y) {
                        board.play(i);

                        #[cfg(debug_assertions)]
                        write!(stdout, "{}|{:?}|", termion::cursor::Goto(30, 16), i).unwrap();
                    };
                }
                _ => (),
            },
            _ => {}
        }
        stdout.flush().unwrap();
    }

    // write!(stdout, "bye");
}
