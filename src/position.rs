use std::ops::Add;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Position {
    pub x: isize,
    pub y: isize,
}

impl Position {
    pub fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    pub fn translate(x: u16, y: u16) -> Option<Self> {
        // TODO: coordinates changed
        let x1 = (x as f32 - 2.0) / 5.0;
        let x2 = x1.ceil();
        let x3 = x1 - x2;

        if y % 2 == 0 && x3 < -0.21 {
            Some(Position::new(x2 as isize, (y / 2) as isize))
        } else {
            None
        }
    }

    pub fn neighbours(&self) -> Vec<Self> {
        let Self { x, y } = *self;

        vec![
            (x + 1, y).into(),
            (x, y + 1).into(),
            (x - 1, y).into(),
            (x, y - 1).into(),
        ]

        // ditch invalids
        // let filtered_v: Vec<Indices> = v
        //     .into_iter()
        //     .filter(|i| (i.0 as u8 >= 1 || i.1 as u8 >= 1) && (i.0 as u8 <= 7 || i.1 as u8 <= 7))
        //     .collect();
        // filtered_v
    }
}

impl From<(isize, isize)> for Position {
    fn from(pos: (isize, isize)) -> Self {
        Self::new(pos.0, pos.1)
    }
}

impl<'a, 'b> Add<&'b Position> for &'a Position {
    type Output = Position;

    fn add(self, rhs: &'b Position) -> Self::Output {
        Position {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_neighbours() {
        let pos = Position::new(1, 1);
        let neighbours = pos.neighbours();
        assert_eq!(neighbours.len(), 4);
    }
}
