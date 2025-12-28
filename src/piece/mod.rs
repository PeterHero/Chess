mod piece_type;
mod team;

use piece_type::PieceType;
use team::Team;

use std::{fmt::Debug, str::FromStr, vec};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Piece {
    piece_type: PieceType,
    team: Team,
}

impl Piece {
    pub fn offsets(self) -> Vec<(isize, isize)> {
        match self.piece_type {
            PieceType::King => {
                let mut v = vec![];
                for i in -1..=1 {
                    for j in -1..=1 {
                        if i != 0 && j != 0 {
                            v.push((i, j));
                        }
                    }
                }
                v
                // TODO: castling
            }
            PieceType::Queen => {
                let mut v = vec![];
                for i in 1..=7 {
                    // diagonals
                    v.push((i, i));
                    v.push((i, -i));
                    v.push((-i, i));
                    v.push((-i, -i));
                    // horizontal and vertical
                    v.push((0, i));
                    v.push((0, -i));
                    v.push((i, 0));
                    v.push((-i, 0));
                }
                v
            }
            PieceType::Rook => {
                let mut v = vec![];
                for i in 1..=7 {
                    v.push((0, i));
                    v.push((0, -i));
                    v.push((i, 0));
                    v.push((-i, 0));
                }
                v
            }
            PieceType::Knight => {
                vec![
                    (2, 1),
                    (2, -1),
                    (-2, 1),
                    (-2, -1),
                    (1, 2),
                    (1, -2),
                    (-1, 2),
                    (-1, -2),
                ]
            }
            PieceType::Bishop => {
                let mut v = vec![];
                for i in 1..=7 {
                    v.push((i, i));
                    v.push((i, -i));
                    v.push((-i, i));
                    v.push((-i, -i));
                }
                v
            }
            PieceType::Pawn => {
                // TODO: en passant
                vec![(self.team.direction(), 0)]
            }
        }
    }
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
