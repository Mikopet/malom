use super::*;

#[derive(Debug, Clone, Copy)]
pub struct Token(pub &'static dyn Color);

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}⬤", Fg(self.0))
    }
}

impl PartialEq for Token {
    fn eq(&self, other: &Self) -> bool {
        let c1 = format!("{:?}", self.0);
        let c2 = format!("{:?}", other.0);
        c1 == c2
    }
}
