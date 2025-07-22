use crate::*;

impl std::fmt::Display for Field {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.vacancy {
            None => write!(f, "◌"),
            Some(token) => write!(f, "{token}"),
        }
    }
}
