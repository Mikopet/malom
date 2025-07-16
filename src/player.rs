use std::fmt::Display;

#[derive(Clone, Copy, Debug)]
pub enum Player {
    None,
    White(u8),
    Black(u8),
}

impl Player {
    pub fn use_token(&self) -> Self {
        match self {
            Player::None => unreachable!(),
            Player::White(t @ 1..) => Player::White(t - 1),
            Player::Black(t @ 1..) => Player::Black(t - 1),
            Player::White(_) => Player::White(0),
            Player::Black(_) => Player::Black(0),
        }
    }
}

impl Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let char = match self {
            Player::None => ' ',
            Player::White(_) => 'W',
            Player::Black(_) => 'B',
        };

        write!(f, "{char}")
    }
}
