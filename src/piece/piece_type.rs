use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum PieceType {
    King,
    Queen,
    Rook,
    Knight,
    Bishop,
    Pawn,
}

impl fmt::Display for PieceType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::King => f.write_str("King"),
            Self::Queen => f.write_str("Queen"),
            Self::Rook => f.write_str("Rook"),
            Self::Knight => f.write_str("Knight"),
            Self::Bishop => f.write_str("Bishop"),
            Self::Pawn => f.write_str("Pawn"),
        }
    }
}
