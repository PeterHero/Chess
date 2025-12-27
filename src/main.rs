use chess::Board;

fn main() {
    let board = Board::default();
    println!("{board}");
    let moves = board.moves(1, 0);
    println!("{:?}", moves);
}
