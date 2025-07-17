use std::fmt::Display;
use termion::color::*;

use crate::player::Player;

#[derive(Clone, Copy, Debug)]
pub enum Field {
    Empty,
    Taken(Player),
}

impl Display for Field {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let char = match self {
            Field::Empty => "O".to_string(),
            Field::Taken(player) => format!("{player}"),
        };

        write!(f, "{char}")?;
        // self.write_fg(f)?;
        self.write_bg(f)
    }
}

impl Color for Field {
    fn write_fg(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(LightBlack.fg_str())
    }

    fn write_bg(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(LightYellow.bg_str())
    }
}
