use crate::*;

impl std::fmt::Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:?}: points({}), hand_count({})",
            self.color,
            self.points(),
            self.hand_count(),
        )
    }
}
