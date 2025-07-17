use super::*;

#[cfg(debug_assertions)]
const PLACEABLE: u8 = 4;
#[cfg(not(debug_assertions))]
const PLACEABLE: u8 = 9;

#[derive(Debug)]
pub struct Player {
    pub token: Token,
    placeable: u8,
    removable: u8,
}

impl Player {
    pub fn new<C: termion::color::Color>(c: &'static C) -> Self {
        Self {
            token: Token(c),
            placeable: PLACEABLE,
            removable: 0,
        }
    }

    pub fn free_tokens(&self) -> u8 {
        self.placeable
    }

    pub fn use_token(&mut self) -> Token {
        if self.placeable > 0 {
            self.placeable -= 1;
        }

        self.token
    }
}

impl std::fmt::Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:?}: place({}), remove({})",
            self.token.0, self.placeable, self.removable
        )
    }
}
