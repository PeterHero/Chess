#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Team {
    White,
    Black,
}

impl Team {
    pub const fn direction(self) -> isize {
        match self {
            Self::White => -1,
            Self::Black => 1,
        }
    }
}
