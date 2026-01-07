use crate::piece::team::Side;
use crate::{Board, piece::Piece};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Pos {
    file: usize,
    rank: usize,
}

impl Pos {
    #[must_use]
    pub fn new(rank: isize, file: isize) -> Option<Self> {
        let rank = usize::try_from(rank).ok()?;
        let file = usize::try_from(file).ok()?;
        if rank < 8 && file < 8 {
            Some(Self { file, rank })
        } else {
            None
        }
    }

    #[must_use]
    pub fn checked_add(self, (r, c): (isize, isize)) -> Option<Self> {
        Self::new(
            isize::try_from(self.rank).ok()? + r,
            isize::try_from(self.file).ok()? + c,
        )
    }

    #[must_use]
    pub const fn rank(&self) -> usize {
        self.rank
    }

    #[must_use]
    pub const fn file(&self) -> usize {
        self.file
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Square {
    pos: Pos,
    content: Option<Piece>,
}

impl Square {
    #[must_use]
    pub const fn new<S: Side + Clone>(pos: Pos, board: &Board<S>) -> Self {
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
