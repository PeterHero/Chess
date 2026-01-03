#![warn(
    clippy::pedantic,
    clippy::nursery,
    clippy::unwrap_used,
    clippy::expect_used
)]

mod board;
mod movement;
mod piece;
mod square;

pub use board::Board;
pub use piece::team::Team;
pub use square::{Pos, Square};
