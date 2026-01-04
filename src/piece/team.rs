#[derive(Clone, Debug, PartialEq, Eq)]
pub struct White;
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Black;

pub trait Side {
    const TEAM: Team;
    type Other: Side + Clone;
}

impl Side for White {
    const TEAM: Team = Team::White;
    type Other = Black;
}

impl Side for Black {
    const TEAM: Team = Team::Black;
    type Other = White;
}

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
