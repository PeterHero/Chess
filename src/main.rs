use chess::Board;
use chess::Pos;

fn main() {
    let mut board = Board::default();
    println!("{board}");
    if let Some(pos) = Pos::new(1, 0) {
        let moves = board.moves(pos);
        println!("{:#?}", moves);
        board.apply(&moves[0]);
    }
    println!("{board}");
}
