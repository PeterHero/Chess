use crate::{Pos, square::Square};

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
pub struct LegalMove {
    from: Square,
    to: Square,
}

impl PossibleMove {
    pub(crate) const fn new(from: Square, to: Square) -> Self {
        Self { from, to }
    }
}

impl LegalMove {
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
