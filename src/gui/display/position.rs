use crate::*;

impl std::fmt::Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            Goto((self.x + 4) as u16 * 5 - 1, (self.y + 4) as u16 * 2)
        )
    }
}
