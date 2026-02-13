use std::collections::HashSet;
use std::marker::PhantomData;
use std::str::FromStr;

use crate::movement::{LegalMove, Move, PossibleMove};
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
        self.board[pos.rank()][pos.file()]
    }

    const fn set(&mut self, pos: Pos, content: Option<Piece>) {
        self.board[pos.rank()][pos.file()] = content;
    }

    #[must_use]
    pub const fn team(&self) -> Team {
        S::TEAM
    }

    fn is_empty_between(&self, from: Pos, to: Pos) -> bool {
        let Ok(to_rank) = isize::try_from(to.rank()) else {
            return false;
        };
        let Ok(from_rank) = isize::try_from(from.rank()) else {
            return false;
        };
        let Ok(to_file) = isize::try_from(to.file()) else {
            return false;
        };
        let Ok(from_file) = isize::try_from(from.file()) else {
            return false;
        };

        let rank_diff = to_rank - from_rank;
        let file_diff = to_file - from_file;
        let rank_dir = match rank_diff {
            0 => 0,
            n => n / n.abs(),
        };
        let file_dir = match file_diff {
            0 => 0,
            n => n / n.abs(),
        };

        let Some(mut pos) = from.checked_add((rank_dir, file_dir)) else {
            return false;
        };
        while pos != to {
            if self.at(pos).is_some() {
                return false;
            }
            let Some(new_pos) = pos.checked_add((rank_dir, file_dir)) else {
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
                PieceType::Pawn => {
                    if raw_move.from.file() == raw_move.to.file() {
                        self.at(raw_move.to).is_none()
                    } else {
                        self.at(raw_move.to).is_some()
                    }
                }
                _ => true,
            })
            .map(|raw_move| {
                let from = Square::new(raw_move.from, self);
                let to = Square::new(raw_move.to, self);
                let move_data = match self.at(raw_move.to) {
                    Some(_) => Move::Capture { from, to },
                    None => Move::Simple { from, to },
                };
                PossibleMove::new(move_data)
            })
            .collect()
    }

    fn enumerate_pieces(&self, team: Team) -> HashSet<Square> {
        let mut pieces = HashSet::new();
        for rank in 0..8 {
            for file in 0..8 {
                if let Some(pos) = Pos::new(rank, file) {
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
                attacked.insert(mv.data().to());
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
                .map(|m| LegalMove::new(m.data()))
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
        let Some(piece) = mv.data().from().content() else {
            return false;
        };

        // board after applied move
        let mut new_board = self.clone();
        new_board.set(mv.data().from().pos(), None);
        new_board.set(mv.data().to().pos(), mv.data().from().content());

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
    pub fn apply_move(&self, mv: &LegalMove<S>) -> Board<S::Other> {
        let mut new_board = Board {
            board: self.board,
            _side: PhantomData,
        };

        new_board.set(mv.data().from().pos(), None);
        new_board.set(
            mv.data().to().pos(),
            mv.data().from().content().map(Piece::touch_piece),
        );
        new_board
    }
}

impl<S: Side + Clone> FromStr for Board<S> {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut b = Self::new();
        let mut ranks = s.lines();
        for i in 0..8 {
            let rank = ranks
                .next()
                .ok_or_else(|| format!("Missing rank {}!", i + 1))?;
            let mut rank = rank.split(',');
            for j in 0..8 {
                let square = rank
                    .next()
                    .ok_or_else(|| format!("Missing square in rank {} file {}!", i + 1, j + 1))?;
                b.board[i][j] = match square {
                    EMPTY_SQUARE => None,
                    str => Some(Piece::from_str(str)?),
                }
            }
            if rank.next().is_some() {
                return Err(format!("Too many squares in rank {}!", i + 1));
            }
        }
        if ranks.next().is_some() {
            return Err("Too many ranks!".to_string());
        }

        Ok(b)
    }
}

impl Default for Board<White> {
    fn default() -> Self {
        match Self::from_str(concat!(
            "wR,wN,wB,wQ,wK,wB,wN,wR\n",
            "wP,wP,wP,wP,wP,wP,wP,wP\n",
            "  ,  ,  ,  ,  ,  ,  ,  \n",
            "  ,  ,  ,  ,  ,  ,  ,  \n",
            "  ,  ,  ,  ,  ,  ,  ,  \n",
            "  ,  ,  ,  ,  ,  ,  ,  \n",
            "bP,bP,bP,bP,bP,bP,bP,bP\n",
            "bR,bN,bB,bQ,bK,bB,bN,bR"
        )) {
            Ok(board) => board,
            Err(err) => panic!("{err}"),
        }
    }
}

impl<S: Side + Clone> std::fmt::Display for Board<S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut str = String::new();
        for rank_i in (0..=7).rev() {
            // for rank in &self.board {
            let rank = &self.board[rank_i];
            str += &(rank_i + 1).to_string();
            str += "|";
            for square in rank {
                if let Some(piece) = square {
                    str += &piece.to_string();
                } else {
                    str += EMPTY_SQUARE;
                }
                str += "|";
            }
            str += "\n";
        }
        str += "   a  b  c  d  e  f  g  h";

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
