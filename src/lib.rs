#![warn(
    clippy::pedantic,
    clippy::nursery,
    clippy::unwrap_used,
    clippy::expect_used
)]

use std::{fmt::Debug, str::FromStr};

#[derive(Clone, Copy, Debug, PartialEq)]
enum PieceType {
    King,
    Queen,
    Rook,
    Knight,
    Bishop,
    Pawn,
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Team {
    White,
    Black,
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Piece {
    piece_type: PieceType,
    team: Team,
}

impl FromStr for Piece {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut s = s.chars();
        let team = s.next().ok_or("Missing team when parsing Piece")?;
        let team = match team {
            'w' => Team::White,
            'b' => Team::Black,
            c => return Err(format!("Unknown Team {c}")),
        };
        let piece_type = s.next().ok_or("Missing piece type when parsing Piece")?;
        let piece_type = match piece_type {
            'K' => PieceType::King,
            'Q' => PieceType::Queen,
            'R' => PieceType::Rook,
            'N' => PieceType::Knight,
            'B' => PieceType::Bishop,
            'P' => PieceType::Pawn,
            c => return Err(format!("Uknown Piece type {c}")),
        };
        if s.next().is_some() {
            return Err("Too many characters for Piece".to_string());
        }
        Ok(Self { piece_type, team })
    }
}

impl std::fmt::Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let team = match self.team {
            Team::White => 'w',
            Team::Black => 'b',
        };
        let piece_type = match self.piece_type {
            PieceType::King => 'K',
            PieceType::Queen => 'Q',
            PieceType::Rook => 'R',
            PieceType::Knight => 'N',
            PieceType::Bishop => 'B',
            PieceType::Pawn => 'P',
        };
        let str = format!("{team}{piece_type}");
        f.write_str(&str)
    }
}

const EMPTY_SQUARE: &str = "  ";

#[derive(Debug, PartialEq)]
pub struct Board {
    board: [[Option<Piece>; 8]; 8],
}

impl Board {
    const fn new() -> Self {
        Self {
            board: [[None; 8]; 8],
        }
    }
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
}
