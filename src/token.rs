use super::*;

#[derive(Debug, Clone, Copy)]
pub struct Token(pub &'static dyn Color);

impl PartialEq for Token {
    fn eq(&self, other: &Self) -> bool {
        let c1 = format!("{:?}", self.0);
        let c2 = format!("{:?}", other.0);
        c1 == c2
    }
}

impl From<&'static dyn Color> for Token {
    fn from(color: &'static dyn Color) -> Self {
        Self(color)
    }
}
