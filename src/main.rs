use chess::Board;
use chess::Team;

fn main() {
    let mut board = Board::default();
    println!("{board}");

    let mut team = Team::White;
    // for _ in 0..300 {
    loop {
        let moves = board.team_legal_moves(team);
        if moves.is_empty() {
            println!("{:?} has no legal moves, GG!", team);
            break;
        }
        board.apply_move(moves[0]);
        team = team.enemy();
        println!("{board}");
    }
}
