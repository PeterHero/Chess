pub mod piece_type;
pub mod team;

use piece_type::PieceType;
use team::Team;

use std::{fmt::Debug, str::FromStr, vec};

use crate::{Pos, movement::RawMove};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[allow(clippy::struct_field_names)]
pub struct Piece {
    piece_type: PieceType,
    team: Team,
    has_moved: bool,
}

fn checked_push(vec: &mut Vec<RawMove>, from: Pos, (c, r): (isize, isize)) {
    if let Some(to) = Pos::checked_add(from, (c, r)) {
        vec.push(RawMove { from, to });
    }
}

impl Piece {
    pub fn raw_moves(self, from: Pos) -> Vec<RawMove> {
        match self.piece_type {
            PieceType::King => {
                let mut v = vec![];
                for i in -1..=1 {
                    for j in -1..=1 {
                        if i != 0 && j != 0 {
                            checked_push(&mut v, from, (i, j));
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
                    checked_push(&mut v, from, (i, i));
                    checked_push(&mut v, from, (i, -i));
                    checked_push(&mut v, from, (-i, i));
                    checked_push(&mut v, from, (-i, -i));
                    // horizontal and vertical
                    checked_push(&mut v, from, (0, i));
                    checked_push(&mut v, from, (0, -i));
                    checked_push(&mut v, from, (i, 0));
                    checked_push(&mut v, from, (-i, 0));
                }
                v
            }
            PieceType::Rook => {
                let mut v = vec![];
                for i in 1..=7 {
                    checked_push(&mut v, from, (0, i));
                    checked_push(&mut v, from, (0, -i));
                    checked_push(&mut v, from, (i, 0));
                    checked_push(&mut v, from, (-i, 0));
                }
                v
            }
            PieceType::Knight => {
                let offsets = vec![
                    (2, 1),
                    (2, -1),
                    (-2, 1),
                    (-2, -1),
                    (1, 2),
                    (1, -2),
                    (-1, 2),
                    (-1, -2),
                ];

                let mut v = vec![];
                for (i, j) in offsets {
                    checked_push(&mut v, from, (i, j));
                }
                v
            }
            PieceType::Bishop => {
                let mut v = vec![];
                for i in 1..=7 {
                    checked_push(&mut v, from, (i, i));
                    checked_push(&mut v, from, (i, -i));
                    checked_push(&mut v, from, (-i, i));
                    checked_push(&mut v, from, (-i, -i));
                }
                v
            }
            PieceType::Pawn => {
                // TODO: en passant
                // TODO: transformation on last rank
                let mut v = vec![];
                checked_push(&mut v, from, (self.team.direction(), 0));
                checked_push(&mut v, from, (self.team.direction(), 1));
                checked_push(&mut v, from, (self.team.direction(), -1));
                if !self.has_moved {
                    checked_push(&mut v, from, (2 * self.team.direction(), 0));
                }
                v
            }
        }
    }

    pub const fn team(self) -> Team {
        self.team
    }

    pub const fn piece_type(self) -> PieceType {
        self.piece_type
    }

    pub const fn has_moved(self) -> bool {
        self.has_moved
    }

    pub const fn touch_piece(self) -> Self {
        Self {
            piece_type: self.piece_type,
            team: self.team,
            has_moved: true,
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
        Ok(Self {
            piece_type,
            team,
            has_moved: false,
        })
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
