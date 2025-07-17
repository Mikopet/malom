use super::*;

#[derive(Debug)]
pub enum Field {
    Empty,
    Taken(Token),
}

impl Field {
    pub fn empty(&self) -> bool {
        match self {
            Field::Empty => true,
            _ => false,
        }
    }
}

impl std::fmt::Display for Field {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Field::Empty => write!(f, "◌"),
            Field::Taken(token) => write!(f, "{token}"),
        }
    }
}
