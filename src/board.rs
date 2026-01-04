use std::collections::HashSet;
use std::marker::PhantomData;
use std::str::FromStr;

use crate::movement::{LegalMove, PossibleMove};
use crate::piece::Piece;
use crate::piece::piece_type::PieceType;
use crate::piece::team::{Side, Team, White};
use crate::square::{Pos, Square};

const EMPTY_SQUARE: &str = "  ";

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Board<S: Side + Clone> {
    board: [[Option<Piece>; 8]; 8],
    _side: PhantomData<S>,
}

impl<S: Side + Clone> Board<S> {
    const fn new() -> Self {
        Self {
            board: [[None; 8]; 8],
            _side: PhantomData,
        }
    }

    #[must_use]
    pub const fn at(&self, pos: Pos) -> Option<Piece> {
        self.board[pos.row()][pos.col()]
    }

    const fn set(&mut self, pos: Pos, content: Option<Piece>) {
        self.board[pos.row()][pos.col()] = content;
    }

    #[must_use]
    pub const fn team(&self) -> Team {
        S::TEAM
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
            if self.at(pos).is_some() {
                return false;
            }
            let Some(new_pos) = pos.checked_add((row_dir, col_dir)) else {
                return false;
            };
            pos = new_pos;
        }
        true
    }

    fn possible_moves(&self, sq: Square) -> Vec<PossibleMove> {
        let Some(piece) = sq.content() else {
            return vec![];
        };

        piece
            .raw_moves(sq.pos())
            .into_iter()
            .filter(|raw_move| {
                self.at(raw_move.to)
                    .is_none_or(|to_piece| to_piece.team() != piece.team())
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

    fn enumerate_pieces(&self, team: Team) -> HashSet<Square> {
        let mut pieces = HashSet::new();
        for row in 0..8 {
            for col in 0..8 {
                if let Some(pos) = Pos::new(row, col) {
                    let sq = Square::new(pos, self);
                    if let Some(piece) = sq.content()
                        && piece.team() == team
                    {
                        pieces.insert(sq);
                    }
                }
            }
        }
        pieces
    }

    fn attacked_squares(&self, team: Team) -> HashSet<Square> {
        let team_pieces = self.enumerate_pieces(team);
        let mut attacked = HashSet::new();
        for sq in team_pieces {
            let moves = self.possible_moves(sq);
            for mv in moves {
                attacked.insert(mv.to());
            }
        }

        attacked
    }

    #[must_use]
    pub fn legal_moves_sq(&self, sq: Square) -> Vec<LegalMove<S>> {
        if let Some(piece) = sq.content()
            && piece.team() == S::TEAM
        {
            self.possible_moves(sq)
                .into_iter()
                .filter(|m| self.is_legal(m))
                .map(|m| LegalMove::new(m.from(), m.to()))
                .collect()
        } else {
            vec![]
        }
    }

    #[must_use]
    pub fn team_legal_moves(&self) -> Vec<LegalMove<S>> {
        let team_pieces = self.enumerate_pieces(S::TEAM);
        let mut moves = vec![];
        for sq in team_pieces {
            for mv in self.legal_moves_sq(sq) {
                moves.push(mv);
            }
        }

        moves
    }

    fn is_legal(&self, mv: &PossibleMove) -> bool {
        let Some(piece) = mv.from().content() else {
            return false;
        };

        // board after applied move
        let mut new_board = self.clone();
        new_board.set(mv.from().pos(), None);
        new_board.set(mv.to().pos(), mv.from().content());

        let attacked = new_board.attacked_squares(piece.team().enemy());
        if attacked.iter().any(|sq| {
            sq.content()
                .is_some_and(|p| p.piece_type() == PieceType::King)
        }) {
            return false;
        }
        true
    }

    #[must_use]
    pub const fn apply_move(&self, mv: &LegalMove<S>) -> Board<S::Other> {
        let mut new_board = Board {
            board: self.board,
            _side: PhantomData,
        };

        new_board.set(mv.from().pos(), None);
        new_board.set(mv.to().pos(), mv.from().content());
        new_board
    }
}

impl<S: Side + Clone> FromStr for Board<S> {
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

impl Default for Board<White> {
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

impl<S: Side + Clone> std::fmt::Display for Board<S> {
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

    #[test]
    #[allow(clippy::unwrap_used)]
    fn possible_moves() {
        let mut default = Board::default();
        assert!(
            !dbg!(default.possible_moves(Square::new(Pos::new(1, 0).unwrap(), &default)))
                .is_empty()
        );
        assert!(
            dbg!(default.possible_moves(Square::new(Pos::new(0, 0).unwrap(), &default))).is_empty()
        );

        default.set(Pos::new(1, 0).unwrap(), None);

        assert!(
            !dbg!(default.possible_moves(Square::new(Pos::new(0, 0).unwrap(), &default)))
                .is_empty()
        );
    }
}
