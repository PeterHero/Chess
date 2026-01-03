use crate::{Board, piece::Piece};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Pos {
    row: usize,
    col: usize,
}

impl Pos {
    #[must_use]
    pub fn new(row: isize, col: isize) -> Option<Self> {
        let row = usize::try_from(row).ok()?;
        let col = usize::try_from(col).ok()?;
        if row < 8 && col < 8 {
            Some(Self { row, col })
        } else {
            None
        }
    }

    #[must_use]
    pub fn checked_add(self, (r, c): (isize, isize)) -> Option<Self> {
        Self::new(
            isize::try_from(self.row).ok()? + r,
            isize::try_from(self.col).ok()? + c,
        )
    }

    #[must_use]
    pub const fn row(&self) -> usize {
        self.row
    }

    #[must_use]
    pub const fn col(&self) -> usize {
        self.col
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Square {
    pos: Pos,
    content: Option<Piece>,
}

impl Square {
    #[must_use]
    pub const fn new(pos: Pos, board: &Board) -> Self {
        Self {
            pos,
            content: board.at(pos),
        }
    }

    #[must_use]
    pub const fn pos(&self) -> Pos {
        self.pos
    }

    #[must_use]
    pub const fn content(&self) -> Option<Piece> {
        self.content
    }
}
