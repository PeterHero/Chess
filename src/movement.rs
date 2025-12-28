use crate::square::Square;

#[derive(Debug)]
pub struct Move {
    from: Square,
    to: Square,
}

impl Move {
    pub const fn new(from: Square, to: Square) -> Self {
        Self { from, to }
    }

    pub const fn from(&self) -> &Square {
        &self.from
    }

    pub const fn to(&self) -> &Square {
        &self.to
    }
}
