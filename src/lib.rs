#![warn(
    clippy::pedantic,
    clippy::nursery,
    clippy::unwrap_used,
    clippy::expect_used
)]

mod board;
mod piece;
mod square;

pub use board::Board;
pub use square::Pos;
