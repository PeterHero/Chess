#![warn(
    clippy::pedantic,
    clippy::nursery,
    clippy::unwrap_used,
    clippy::expect_used
)]

use std::{
    fmt::{Debug, Write},
    str::FromStr,
};

#[derive(Clone, Copy, Debug, PartialEq)]
enum Piece {
    King,
    Queen,
    Rook,
    Knight,
    Bishop,
    Pawn,
}

impl FromStr for Piece {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "K" => Ok(Self::King),
            "Q" => Ok(Self::Queen),
            "R" => Ok(Self::Rook),
            "N" => Ok(Self::Knight),
            "B" => Ok(Self::Bishop),
            "P" => Ok(Self::Pawn),
            str => Err(format!("Uknown Piece type {str}")),
        }
    }
}

impl std::fmt::Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Self::King => 'K',
            Self::Queen => 'Q',
            Self::Rook => 'R',
            Self::Knight => 'N',
            Self::Bishop => 'B',
            Self::Pawn => 'P',
        };
        f.write_char(c)
    }
}

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
                    " " => None,
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
            "R,N,B,Q,K,B,N,R\n",
            "P,P,P,P,P,P,P,P\n",
            " , , , , , , , \n",
            " , , , , , , , \n",
            " , , , , , , , \n",
            " , , , , , , , \n",
            "P,P,P,P,P,P,P,P\n",
            "R,N,B,Q,K,B,N,R"
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
                    str += " ";
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
