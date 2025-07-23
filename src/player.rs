use super::*;

#[cfg(debug_assertions)]
const TOKEN_COUNT: usize = 4;
#[cfg(not(debug_assertions))]
const TOKEN_COUNT: usize = 9;

#[derive(Debug)]
pub struct Player {
    pub color: &'static dyn Color,
    hand: Vec<Token>,
    lost: usize,
    remove: isize,
}

impl Player {
    pub fn new<C: Color>(color: &'static C) -> Self {
        Self {
            color,
            hand: vec![Token(color); TOKEN_COUNT],
            lost: 0,
            remove: 0,
        }
    }

    pub fn hand_count(&self) -> usize {
        self.hand.len()
    }

    pub fn points(&self) -> usize {
        TOKEN_COUNT - self.hand_count() - self.lost
    }

    pub fn removes(&self) -> isize {
        self.remove
    }

    pub fn use_token(&mut self) -> Option<Token> {
        self.hand.pop()
    }

    pub fn increase_lost(&mut self) {
        self.lost += 1;
    }

    pub fn change_remove(&mut self, n: isize) {
        self.remove += n;
    }

    pub fn must_remove(&self) -> bool {
        self.remove > 0
    }
}
