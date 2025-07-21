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
    points: u8,
    pub selected: Option<Indices>,
}

impl Player {
    pub fn new<C: termion::color::Color>(c: &'static C) -> Self {
        Self {
            token: Token(c),
            placeable: PLACEABLE,
            removable: 0,
            points: 0,
            selected: None,
        }
    }

    pub fn free_tokens(&self) -> u8 {
        self.placeable
    }
    pub fn points(&self) -> u8 {
        self.points
    }

    pub fn use_token(&mut self) -> Token {
        if self.placeable > 0 {
            self.placeable -= 1;
        }

        self.token
    }

    pub fn add_remove(&mut self, n: u8) {
        self.removable += n
    }
    pub fn remove_remove(&mut self) {
        self.removable -= 1
    }
    pub fn get_remove(&self) -> u8 {
        self.removable
    }
    pub fn add_point(&mut self) {
        self.points += 1;
    }
    pub fn remove_point(&mut self) {
        self.points -= 1
    }
    pub fn set_selected(&mut self, i: Indices) {
        self.selected = Some(i)
    }
    pub fn neighbour(&self, _i: Indices) -> Vec<Indices> {
        let mut v: Vec<Indices> = Vec::new();
        if let Some(i) = self.selected {
            match i {
                (x, y) => {
                    v.push(((x as u16 + 1).into(), y));
                    v.push((x, (y as u16 + 1).into()));
                    v.push(((x as u16 - 1).into(), y));
                    v.push((x, (y as u16 - 1).into()));
                }
            };
            return v;
        }
        let filtered_v: Vec<Indices> = v
            .into_iter()
            .filter(|i| (i.0 as u8 >= 1 || i.1 as u8 >= 1) && (i.0 as u8 <= 7 || i.1 as u8 <= 7))
            .collect();
        filtered_v
    }
}

impl std::fmt::Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:?}: place({}), remove({}), points ({})",
            self.token.0, self.placeable, self.removable, self.points
        )
    }
}
