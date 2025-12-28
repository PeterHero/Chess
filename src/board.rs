use std::str::FromStr;

use crate::piece::Piece;
use crate::square::Pos;

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

    const fn at(&self, pos: &Pos) -> Option<Piece> {
        self.board[pos.row()][pos.col()]
    }

    #[must_use]
    pub fn moves(&self, pos: Pos) -> Vec<Pos> {
        let piece = self.at(&pos);
        let Some(piece) = piece else {
            return vec![];
        };

        piece
            .offsets()
            .iter()
            .filter_map(|(r, c)| pos.checked_add((*r, *c)))
            .collect()
        // TODO: coordinates to Moves
        // TODO: filter legal Moves
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
