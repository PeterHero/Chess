use std::str::FromStr;

use crate::movement::{LegalMove, PossibleMove, RawMove};
use crate::piece::piece_type::PieceType;
use crate::piece::{self, Piece};
use crate::square::{Pos, Square};

const EMPTY_SQUARE: &str = "  ";

#[derive(Debug, PartialEq, Eq)]
pub struct Board {
    board: [[Option<Piece>; 8]; 8],
}

impl Board {
    const fn new() -> Self {
        Self {
            board: [[None; 8]; 8],
        }
    }

    pub const fn at(&self, pos: &Pos) -> Option<Piece> {
        self.board[pos.row()][pos.col()]
    }

    const fn set(&mut self, pos: &Pos, content: Option<Piece>) {
        self.board[pos.row()][pos.col()] = content;
    }

    fn is_empty_between(&self, from: Pos, to: Pos) -> bool {
        let Ok(to_row) = isize::try_from(to.row()) else {
            return false;
        };
        let Ok(from_row) = isize::try_from(from.row()) else {
            return false;
        };
        let Ok(to_col) = isize::try_from(to.col()) else {
            return false;
        };
        let Ok(from_col) = isize::try_from(from.col()) else {
            return false;
        };

        let row_diff = to_row - from_row;
        let col_diff = to_col - from_col;
        let row_dir = match row_diff {
            0 => 0,
            n => n / n.abs(),
        };
        let col_dir = match col_diff {
            0 => 0,
            n => n / n.abs(),
        };

        let Some(mut pos) = from.checked_add((row_dir, col_dir)) else {
            return false;
        };
        while pos != to {
            if self.at(&pos).is_some() {
                return false;
            }
            let Some(new_pos) = pos.checked_add((row_dir, col_dir)) else {
                return false;
            };
            pos = new_pos;
        }
        true
    }

    #[must_use]
    pub fn possible_moves(&self, sq: Square) -> Vec<PossibleMove> {
        let Some(piece) = sq.content() else {
            return vec![];
        };

        piece
            .raw_moves(sq.pos())
            .iter()
            .filter(|raw_move| {
                if let Some(to_piece) = self.at(&raw_move.to)
                    && to_piece.team() == piece.team()
                {
                    false
                } else {
                    true
                }
            })
            .filter(|raw_move| match piece.piece_type() {
                PieceType::Rook | PieceType::Bishop | PieceType::Queen => {
                    self.is_empty_between(sq.pos(), raw_move.to)
                }
                _ => true,
            })
            .map(|raw_move| PossibleMove::new(sq, Square::new(raw_move.to, self)))
            .collect()
    }

    pub fn legal_moves(&self, sq: Square) -> Vec<LegalMove> {
        todo!()
    }

    pub fn is_legal(&self, mv: &PossibleMove) -> bool {
        todo!()
    }

    /*
    #[must_use]
    pub fn moves(&self, pos: Pos) -> Vec<Move> {
        let piece = self.at(&pos);
        let Some(piece) = piece else {
            return vec![];
        };

        piece
            .offsets()
            .iter()
            .filter_map(|(r, c)| pos.checked_add((*r, *c)))
            .map(|to| {
                Move::new(
                    Square::new(pos, self.at(&pos)),
                    Square::new(to, self.at(&to)),
                )
            })
            .collect()
        // TODO: filter legal Moves
    }

    pub const fn apply(&mut self, move_cmd: &Move) {
        self.set(&move_cmd.from().pos(), None);
        self.set(&move_cmd.to().pos(), move_cmd.from().content());
    }
     */
}

impl FromStr for Board {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut b = Self::new();
        let mut rows = s.lines();
        for i in 0..8 {
            let row = rows
                .next()
                .ok_or_else(|| format!("Missing row {}!", i + 1))?;
            let mut row = row.split(',');
            for j in 0..8 {
                let square = row
                    .next()
                    .ok_or_else(|| format!("Missing square in row {} col {}!", i + 1, j + 1))?;
                b.board[i][j] = match square {
                    EMPTY_SQUARE => None,
                    str => Some(Piece::from_str(str)?),
                }
            }
            if row.next().is_some() {
                return Err(format!("Too many squares in row {}!", i + 1));
            }
        }
        if rows.next().is_some() {
            return Err("Too many rows!".to_string());
        }

        Ok(b)
    }
}

impl Default for Board {
    fn default() -> Self {
        match Self::from_str(concat!(
            "bR,bN,bB,bQ,bK,bB,bN,bR\n",
            "bP,bP,bP,bP,bP,bP,bP,bP\n",
            "  ,  ,  ,  ,  ,  ,  ,  \n",
            "  ,  ,  ,  ,  ,  ,  ,  \n",
            "  ,  ,  ,  ,  ,  ,  ,  \n",
            "  ,  ,  ,  ,  ,  ,  ,  \n",
            "wP,wP,wP,wP,wP,wP,wP,wP\n",
            "wR,wN,wB,wQ,wK,wB,wN,wR"
        )) {
            Ok(board) => board,
            Err(err) => panic!("{err}"),
        }
    }
}

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut str = String::new();
        for row in &self.board {
            str += "|";
            for square in row {
                if let Some(piece) = square {
                    str += &piece.to_string();
                } else {
                    str += EMPTY_SQUARE;
                }
                str += "|";
            }
            str += "\n";
        }
        f.write_str(&str)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_board_does_not_panic() {
        let default = Board::default();
        let empty = Board::new();
        assert_ne!(default, empty);
    }

    #[test]
    #[allow(clippy::unwrap_used)]
    fn is_empty_between_on_default() {
        let default = Board::default();
        assert!(default.is_empty_between(Pos::new(1, 1).unwrap(), Pos::new(6, 1).unwrap()));
        assert!(!default.is_empty_between(Pos::new(1, 1).unwrap(), Pos::new(7, 1).unwrap()));
        assert!(default.is_empty_between(Pos::new(1, 1).unwrap(), Pos::new(3, 3).unwrap()));
        assert!(default.is_empty_between(Pos::new(1, 5).unwrap(), Pos::new(5, 1).unwrap()));
    }
}
