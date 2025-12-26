use std::fmt::{Debug, Write};

#[derive(Clone, Copy, Debug)]
enum Piece {
    King,
    Queen,
    Rook,
    Knight,
    Bishop,
    Pawn
}

impl std::fmt::Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Self::King => 'K',
            Self::Queen => 'Q',
            Self::Rook => 'R',
            Self:: Knight => 'N',
            Self::Bishop => 'B',
            Self::Pawn => 'P'
        };
        f.write_char(c)
    }
}

#[derive(Debug)]
struct Board {
    board: [[Option<Piece>; 8]; 8]
}

impl Default for Board {
    fn default() -> Self {
        let mut b = Board {board: [[None; 8]; 8]};
        let base_row = [Some(Piece::Rook), Some(Piece::Knight), Some(Piece::Bishop), Some(Piece::Queen), Some(Piece::King), Some(Piece::Bishop), Some(Piece::Knight), Some(Piece::Rook)];
        b.board[0] = base_row.clone();
        b.board[1] = [Some(Piece::Pawn); 8];
        b.board[6] = [Some(Piece::Pawn); 8];
        b.board[7] = base_row;
        b
    }
}

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut str = String::new();
        for row in &self.board {
            str += "|";
            for square in row {
                let c = match square {
                    Some(piece) => piece.to_string(),
                    None => ' '.to_string()
                };
                str += &c;
                str += "|";
            }
            str += "\n";
        }
        f.write_str(&str)
    }
}

fn main() {
    let board = Board::default();
    println!("{board}");
}
