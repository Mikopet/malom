#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug)]
pub enum Index {
    Invalid = 0,
    A = 1,
    B = 2,
    C = 3,
    D = 4,
    E = 5,
    F = 6,
    G = 7,
}

impl From<u16> for Index {
    fn from(value: u16) -> Self {
        match value {
            1 => Self::A,
            2 => Self::B,
            3 => Self::C,
            4 => Self::D,
            5 => Self::E,
            6 => Self::F,
            7 => Self::G,
            _ => Self::Invalid,
        }
    }
}
