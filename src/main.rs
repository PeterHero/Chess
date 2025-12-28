use chess::Board;
use chess::Pos;

fn main() {
    let board = Board::default();
    println!("{board}");
    if let Some(pos) = Pos::new(1, 0) {
        let moves = board.moves(pos);
        println!("{:?}", moves);
    }
}
