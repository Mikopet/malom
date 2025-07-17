use std::fmt::Display;
use termion::color::*;

#[derive(Clone, Copy, Debug)]
pub enum Player {
    White(u8),
    Black(u8),
}

impl Player {
    pub fn use_token(&self) -> Self {
        match self {
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
            Player::White(_) => 'W',
            Player::Black(_) => 'B',
        };

        self.write_fg(f)?;
        self.write_bg(f)?;
        write!(f, "{char}")
    }
}

impl Color for Player {
    fn write_fg(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Player::White(_) => f.write_str(Black.fg_str()),
            Player::Black(_) => f.write_str(White.fg_str()),
        }
    }

    fn write_bg(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Player::White(_) => f.write_str(White.bg_str()),
            Player::Black(_) => f.write_str(Black.bg_str()),
        }
    }
}
