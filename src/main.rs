use chess::{Black, Board, White};

enum Game {
    White(Board<White>),
    Black(Board<Black>),
}

fn main() {
    let mut game = Game::White(Board::default());
    match game {
        Game::White(ref board) => println!("{board}"),
        Game::Black(ref board) => println!("{board}"),
    }

    for _ in 0..10 {
        // loop {
        game = match game {
            Game::White(board) => {
                let moves = board.team_legal_moves();
                if moves.is_empty() {
                    println!("{:?} has no legal moves, GG!", board.team());
                    break;
                }
                let board = board.apply_move(&moves[0]);
                println!("{board}");
                Game::Black(board)
            }
            Game::Black(board) => {
                let moves = board.team_legal_moves();
                if moves.is_empty() {
                    println!("{:?} has no legal moves, GG!", board.team());
                    break;
                }
                let board = board.apply_move(&moves[0]);
                println!("{board}");
                Game::White(board)
            }
        }
    }
}
