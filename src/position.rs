// TODO: make board size configurable (Config struct)
pub const BOARD_SIZE: usize = 3;
pub const BOARD_RANGE: std::ops::Range<isize> =
    0 - (BOARD_SIZE as isize)..1 + (BOARD_SIZE as isize);

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Position {
    pub x: isize,
    pub y: isize,
}

impl Position {
    pub fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    // TODO: cache it somewhere
    pub fn valid_fields() -> Vec<Self> {
        let mut v = vec![];
        for y in BOARD_RANGE {
            for x in BOARD_RANGE {
                match (x, y) {
                    (0, 0) => continue,
                    (_, 0) => {}
                    (0, _) => {}
                    (k, l) if k == l => {}
                    (k, l) if k == -l => {}
                    _ => continue,
                };
                v.push(Position::new(x, y));
            }
        }
        v
    }

    // TODO: fix this mess
    pub fn translate(x: u16, y: u16) -> Option<Self> {
        let x1 = (x as f32 - 7.0) / 5.0 - 3.0;
        let x2 = x1.ceil();
        let x3 = x1 - x2;

        if y % 2 == 0 && x3 < -0.21 {
            Some(Position::new(x2 as isize, (y / 2) as isize - 4))
        } else {
            None
        }
    }

    pub fn neighbours(&self) -> Vec<Self> {
        let Self { x, y } = *self;

        let x_delta = if x != 0 { x.abs() } else { 1 };
        let y_delta = if y != 0 { y.abs() } else { 1 };

        let mut v = vec![
            (x + y_delta, y).into(),
            (x - y_delta, y).into(),
            (x, y + x_delta).into(),
            (x, y - x_delta).into(),
        ];

        v.retain(|pos| Position::valid_fields().contains(pos));
        v
    }
}

impl From<(isize, isize)> for Position {
    fn from(pos: (isize, isize)) -> Self {
        Self::new(pos.0, pos.1)
    }
}

impl<'a, 'b> std::ops::Add<&'b Position> for &'a Position {
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
    fn test_valid_fields() {
        let fields = Position::valid_fields();
        assert!(fields.contains(&Position::new(1, 1)));
        assert!(fields.contains(&Position::new(3, 3)));
    }

    #[test]
    fn test_invalid_fields() {
        let fields = Position::valid_fields();
        assert!(!fields.contains(&Position::new(0, 0)));
        assert!(!fields.contains(&Position::new(4, 4)));
        assert!(!fields.contains(&Position::new(-1, -2)));
        assert!(!fields.contains(&Position::new(2, 3)));
    }

    #[test]
    fn test_neighbours_top_corner() {
        let pos = Position::new(1, 1);
        assert_eq!(pos.neighbours().len(), 2);
    }
    #[test]
    fn test_neighbours_top_middle() {
        let pos = Position::new(0, -3);
        let n = pos.neighbours();
        dbg!(n);
        assert_eq!(pos.neighbours().len(), 3);
    }

    #[test]
    fn test_neighbours_middle_middle() {
        let pos = Position::new(0, -2);
        assert_eq!(pos.neighbours().len(), 4);
    }

    #[test]
    fn test_neighbours_middle_bottom() {
        let pos = Position::new(0, -1);
        assert_eq!(pos.neighbours().len(), 3);
    }

    #[test]
    fn test_neighbours_middle_corner() {
        let pos = Position::new(2, 2);
        assert_eq!(pos.neighbours().len(), 2);
    }

    #[test]
    fn test_translate() {
        // invalid coordinates
        assert_eq!(Position::translate(4, 3), None);
        assert_eq!(Position::translate(7, 2), None);

        // some valid ones
        assert_eq!(Position::translate(4, 4), Some(Position::new(-3, -2)));
        assert_eq!(Position::translate(19, 8), Some(Position::new(0, 0)));
    }
}
