use std::fmt::Display;

use crate::player::Player;

#[derive(Clone, Copy, Debug)]
pub enum Field {
    // TODO: nem kell
    Invalid,
    Empty,
    Valid(Player),
}

impl Display for Field {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Field::Invalid => write!(f, "   "),
            Field::Empty => write!(f, " O "),
            Field::Valid(player) => write!(f, "({player})"),
        }
    }
}
