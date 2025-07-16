use std::{collections::HashMap, fmt::Display};

use crate::{field::Field, index::Index, player::Player};

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
    fields: HashMap<(Index, Index), Field>,
    pub current_player: Player,
    other_player: Player,
}

impl Board {
    pub fn init() -> Self {
        let mut board = HashMap::new();

        for i in &INDICES {
            for j in &INDICES {
                board.insert(
                    (*i, *j),
                    match (*i, *j) {
                        (Index::D, Index::D) => Field::Invalid,
                        (_, Index::D) => Field::Empty,
                        (Index::D, _) => Field::Empty,
                        (k, l) if k == l => Field::Empty,
                        (k, l) if (8 - k as u8) == (l as u8) => Field::Empty,
                        _ => Field::Invalid,
                    },
                );
            }
        }

        Self {
            fields: board,
            current_player: Player::White(4),
            other_player: Player::Black(4),
        }
    }

    pub fn get(&self, index: (Index, Index)) -> Option<&Field> {
        self.fields.get(&index)
    }

    pub fn play(&mut self, index: (Index, Index)) {
        match self.current_player {
            Player::None => unreachable!(),
            Player::White(t @ 0) => {}
            Player::Black(t @ 0) => {}
            // when player has tokens and just inserting (phase 1)
            player => {
                // dbg!(player);
                if let Some(Field::Empty) = self.get(index) {
                    self.current_player = self.current_player.use_token();
                    self.fields.insert(index, Field::Valid(self.current_player));
                    self.switch_player();
                }
            }
        };
        /*
        if let Some(Field::Valid(player)) = self.get(index) {
            if player == self.current_player {
                // if let Action::Move(target) = self.current_action
                if self.current_action == Action::Move {}

            }

            // if player is cyurrrent,
            // then its a moving ation
        }
        if let Some(Field::Valid(Player::None)) = self.get(index) {
            match self.current_player {
                Player::None => unreachable!(),
                Player::White(t @ 0) => {}
                Player::Black(t @ 0) => {}
                // when player has tokens and just inserting (phase 1)
                _ => {
                    self.current_player = self.current_player.use_token();
                    self.fields.insert(index, Field::Valid(self.current_player));
                }
            };
        }
        // mill()
        */
    }

    fn mill() {}

    pub fn get_by_tui(x: u16, y: u16) -> (Index, Index) {
        let x1 = (x as f32 - 2.0) / 5.0;
        let x2 = x1.ceil();
        let x3 = x1 - x2;
        match (x, y) {
            (_, y) if y % 2 == 1 => (Index::Invalid, Index::Invalid),
            (_, _) if x3 < -0.21 => (Index::from(x2 as u16), Index::from(y / 2)),
            _ => (Index::Invalid, Index::Invalid),
        }
    }

    fn switch_player(&mut self) {
        std::mem::swap(&mut self.current_player, &mut self.other_player);

        // self.current_player = match self.current_player {
        //     Player::None => unreachable!(),
        //     Player::White(b) => {
        //         if let Player::Black(black_left) = self.other_player {
        //             self.other_player = Player::White(b);
        //             Player::Black(black_left)
        //         }
        //     }
        //     Player::Black() => Player::White(),
        // };
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "+{}+\n\r", "=".repeat(35));
        for y in &INDICES {
            write!(f, "|");

            for x in &INDICES {
                let field = self.fields.get(&(*x, *y)).unwrap();
                write!(f, " {field} ");
                if let Index::G = *x {
                    write!(f, "|\n\r");
                    if *y != Index::G {
                        write!(f, "|{}|\n\r", " ".repeat(35));
                    }
                }
            }
        }
        write!(f, "+{}+\n\r", "=".repeat(35));
        write!(f, "Current Player: {:?}", self.current_player)
    }
}
