use super::*;

#[derive(Debug)]
pub struct Field {
    pub vacancy: Option<Token>,
}

impl Field {
    pub fn is_empty(&self) -> bool {
        self.vacancy.is_none()
    }

    pub fn belongs_to(&self, color: &'static dyn Color) -> bool {
        match self.vacancy {
            Some(token) => token == Token(color),
            None => false,
        }
    }

    pub fn vacate(&mut self) -> &Self {
        self.vacancy.take();
        self
    }

    pub fn occupy(&mut self, token: Token) -> &Self {
        self.vacancy = Some(token);
        self
    }
}

impl Default for Field {
    fn default() -> Self {
        Self { vacancy: None }
    }
}
