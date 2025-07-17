use super::*;

#[derive(Debug, Clone, Copy)]
pub struct Token(pub &'static dyn Color);

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}⬤", Fg(self.0))
    }
}
