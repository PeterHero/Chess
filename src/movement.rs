use std::marker::PhantomData;

use crate::{Pos, piece::team::Side, square::Square};

#[derive(Clone, Copy, Debug)]
pub struct RawMove {
    pub from: Pos,
    pub to: Pos,
}

#[derive(Clone, Copy, Debug)]
pub enum Move {
    Simple { from: Square, to: Square },
    Capture { from: Square, to: Square },
    /*
    Castle {
        from: Square,
        to: Square,
        rook_from: Square,
        rook_to: Square,
    },
     */
}

impl Move {
    pub const fn from(&self) -> Square {
        match self {
            Self::Simple { from, to: _ } | Self::Capture { from, to: _ } => *from,
        }
    }
    pub const fn to(&self) -> Square {
        match self {
            Self::Simple { from: _, to } | Self::Capture { from: _, to } => *to,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct PossibleMove {
    move_data: Move,
}

#[derive(Clone, Copy, Debug)]
pub struct LegalMove<S: Side> {
    move_data: Move,
    _side: PhantomData<S>,
}

impl PossibleMove {
    pub(crate) const fn new(move_data: Move) -> Self {
        Self { move_data }
    }

    pub const fn data(&self) -> Move {
        self.move_data
    }
}

impl<S: Side> LegalMove<S> {
    pub(crate) const fn new(move_data: Move) -> Self {
        Self {
            move_data,
            _side: PhantomData,
        }
    }

    #[must_use]
    pub const fn data(&self) -> Move {
        self.move_data
    }
}
