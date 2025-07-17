use std::{collections::HashMap, fmt::Display};
use termion::{color::*, style::Reset};

use crate::{
    field::Field,
    index::{Index, Indices},
    player::Player,
};

const INDICES: [Index; 7] = [
    Index::A,
    Index::B,
    Index::C,
    Index::D,
    Index::E,
    Index::F,
    Index::G,
];

#[derive(Debug)]
pub struct Board {
    fields: HashMap<Indices, Field>,
    pub current_player: Player,
    other_player: Player,
    highlight: Option<Indices>,
}

impl Board {
    pub fn init() -> Self {
        let mut board = HashMap::new();

        for i in &INDICES {
            for j in &INDICES {
                let indices = (i, j);
                let field = match indices {
                    (Index::D, Index::D) => continue,
                    (_, Index::D) => Field::Empty,
                    (Index::D, _) => Field::Empty,
                    (k, l) if k == l => Field::Empty,
                    (k, l) if (8 - *k as u8) == (*l as u8) => Field::Empty,
                    _ => continue,
                };
                board.insert((*i, *j), field); // empty
            }
        }

        Self {
            fields: board,
            current_player: Player::White(9),
            other_player: Player::Black(9),
            highlight: None,
        }
    }

    pub fn get(&self, index: Indices) -> Option<&Field> {
        self.fields.get(&index)
    }

    pub fn play(&mut self, index: Indices) {
        if let Some(field) = self.get(index) {
            match self.current_player {
                // Phase 2: Moving pieces
                Player::White(_t @ 0) => {}
                Player::Black(_t @ 0) => {}
                // Phase 1: Placing Pieces
                _ => {
                    if let Field::Empty = field {
                        self.place_piece(index);
                        if !self.mill() {
                            //malom dolgok
                            self.switch_player();
                        }
                    }
                }
            }
        };
    }

    fn place_piece(&mut self, index: Indices) {
        self.current_player = self.current_player.use_token();
        self.fields.insert(index, Field::Taken(self.current_player));
        self.highlight = Some(index);
    }

    // fn move_piece(&mut self, index: Indices) {
    //     if let Field::Taken(p) = self.get(index) {
    //         if p == self.current_player {}
    //     }
    // }

    fn mill(&self) -> bool {
        false
    }

    pub fn get_indices(x: u16, y: u16) -> Option<Indices> {
        let x1 = (x as f32 - 2.0) / 5.0;
        let x2 = x1.ceil();
        let x3 = x1 - x2;

        if y % 2 == 0 && x3 < -0.21 {
            Some((Index::from(x2 as u16), Index::from(y / 2)))
        } else {
            None
        }
    }

    fn switch_player(&mut self) {
        std::mem::swap(&mut self.current_player, &mut self.other_player);
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.write_fg(f)?;
        self.write_bg(f)?;
        write!(f, "+{}+\n\r", "=".repeat(35))?;
        for y in &INDICES {
            write!(f, "|")?;

            for x in &INDICES {
                let index = (*x, *y);
                let hlc = if self.highlight == Some(index) {
                    Green.fg_str()
                } else {
                    LightBlack.fg_str()
                };
                match self.get(index) {
                    Some(field) => match field {
                        Field::Empty => write!(f, "  {field}  ")?,
                        Field::Taken(player) => write!(f, " {c}({field}{c}) ", c = hlc)?,
                    },
                    None => write!(f, "     ")?,
                };
                self.write_fg(f)?;
                self.write_bg(f)?;

                if let Index::G = *x {
                    write!(f, "|\n\r")?;
                    if *y != Index::G {
                        write!(f, "|{}|\n\r", " ".repeat(35))?;
                    }
                }
            }
        }
        write!(f, "+{}+\n\r", "=".repeat(35))?;
        write!(f, "{}Current Player: {:?}\n\r", Reset, self.current_player)
    }
}

impl Color for Board {
    fn write_fg(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(LightBlack.fg_str())
    }

    fn write_bg(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(LightYellow.bg_str())
    }
}
