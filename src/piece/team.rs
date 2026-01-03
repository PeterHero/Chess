#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Team {
    White,
    Black,
}

impl Team {
    #[must_use]
    pub const fn direction(self) -> isize {
        match self {
            Self::White => -1,
            Self::Black => 1,
        }
    }

    #[must_use]
    pub const fn enemy(self) -> Self {
        match self {
            Self::White => Self::Black,
            Self::Black => Self::White,
        }
    }
}
