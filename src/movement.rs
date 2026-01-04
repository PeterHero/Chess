use std::marker::PhantomData;

use crate::{Pos, piece::team::Side, square::Square};

#[derive(Clone, Copy, Debug)]
pub struct RawMove {
    pub from: Pos,
    pub to: Pos,
}

#[derive(Clone, Copy, Debug)]
pub struct PossibleMove {
    from: Square,
    to: Square,
}

#[derive(Clone, Copy, Debug)]
pub struct LegalMove<S: Side> {
    from: Square,
    to: Square,
    _side: PhantomData<S>,
}

impl PossibleMove {
    pub(crate) const fn new(from: Square, to: Square) -> Self {
        Self { from, to }
    }

    pub const fn from(&self) -> Square {
        self.from
    }

    pub const fn to(&self) -> Square {
        self.to
    }
}

impl<S: Side> LegalMove<S> {
    pub(crate) const fn new(from: Square, to: Square) -> Self {
        Self {
            from,
            to,
            _side: PhantomData,
        }
    }

    pub const fn from(&self) -> Square {
        self.from
    }

    pub const fn to(&self) -> Square {
        self.to
    }
}
