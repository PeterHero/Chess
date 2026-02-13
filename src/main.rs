use std::io;

use chess::{Black, Board, LegalMove, PieceType, Pos, RawMove, Side, Square, White};

enum Game {
    White(Board<White>),
    Black(Board<Black>),
}

enum MoveType {
    Simple,
    Capture,
}

fn parse_piece(c: char) -> Result<PieceType, String> {
    match c {
        'K' => Ok(PieceType::King),
        'Q' => Ok(PieceType::Queen),
        'R' => Ok(PieceType::Rook),
        'N' => Ok(PieceType::Knight),
        'B' => Ok(PieceType::Bishop),
        'P' => Ok(PieceType::Pawn),
        _ => Err(format!("Uknown piece '{c}', must be one of K,Q,R,N,B,P")),
    }
}

fn parse_file(c: char) -> Result<isize, String> {
    match c {
        'a' => Ok(0),
        'b' => Ok(1),
        'c' => Ok(2),
        'd' => Ok(3),
        'e' => Ok(4),
        'f' => Ok(5),
        'g' => Ok(6),
        'h' => Ok(7),
        _ => Err(format!("Uknown file '{c}', must be a-h")),
    }
}

fn parse_rank(c: char) -> Result<isize, String> {
    let rank = c
        .to_string()
        .parse::<isize>()
        .map_err(|_| "Rank must be a number")?;
    if !(1..=8).contains(&rank) {
        return Err("Rank must be 1-8".to_string());
    }
    Ok(rank - 1)
}

fn parse_move_type(c: char) -> Result<MoveType, String> {
    match c {
        '-' => Ok(MoveType::Simple),
        'x' => Ok(MoveType::Capture),
        _ => Err("Move type must be '-' or 'x'".to_string()),
    }
}

fn parse_move(input: &str) -> Result<(PieceType, RawMove, MoveType), String> {
    let mut c = input.chars();

    let piece = c.next().ok_or("Missing Piece")?;
    let piece = parse_piece(piece)?;

    let file_from = c.next().ok_or("Missing file of starting square")?;
    let file_from = parse_file(file_from)?;
    let rank_from = c.next().ok_or("Missing rank of starting square")?;
    let rank_from = parse_rank(rank_from)?;

    let del = c.next().ok_or("Missing type of move")?;
    let mv_type = parse_move_type(del)?;

    let file_to = c.next().ok_or("Missing file of ending square")?;
    let file_to = parse_file(file_to)?;
    let rank_to = c.next().ok_or("Missing rank of ending square")?;
    let rank_to = parse_rank(rank_to)?;
    if c.next().is_some() {
        return Err("Too many characters in input".to_string());
    }

    let from =
        Pos::new(rank_from, file_from).ok_or("Error creating position from file and rank")?;
    let to = Pos::new(rank_to, file_to).ok_or("Error creating position from file and rank")?;
    let mv = RawMove { from, to };

    Ok((piece, mv, mv_type))
}

fn user_move<S: Side + Clone>(board: &Board<S>) -> Result<LegalMove<S>, String> {
    println!("{} on move: ", S::TEAM);

    let mut mv = String::new();
    io::stdin().read_line(&mut mv).expect("Failed to read line");
    println!("Playing {}", mv);

    let (piece, mv, mv_type) = parse_move(mv.trim())?;

    let from_piece = board
        .at(mv.from)
        .ok_or("Starting square must not be empty")?;

    if from_piece.team() != S::TEAM {
        return Err(format!(
            "Starting square must have piece of side {}",
            S::TEAM
        ));
    }

    if from_piece.piece_type() != piece {
        return Err(format!(
            "Starting square must have a piece of type {}",
            piece
        ));
    }

    let moves = board.legal_moves_sq(Square::new(mv.from, board));

    if moves.is_empty() {
        return Err("The piece has no legal moves".to_string());
    }

    let legal_move = moves.into_iter().find(|legal_move| {
        legal_move.data().from() == Square::new(mv.from, board)
            && legal_move.data().to() == Square::new(mv.to, board)
    });

    let legal_move = legal_move.ok_or("The move to ending square is not legal")?;

    match (mv_type, legal_move.data().to().content()) {
        (MoveType::Simple, Some(_)) => Err("Specified non-capture move type: '-', but ending square is not empty. Use 'x' instead.".to_string()),
        (MoveType::Capture, None) => Err("Specified caputre move type: 'x', but ending square is empty. Use '-' instead.".to_string()),
        (MoveType::Simple, None) => Ok(legal_move),
        (MoveType::Capture, Some(_)) => Ok(legal_move),
    }
}

fn main() {
    let mut game = Game::White(Board::default());
    match game {
        Game::White(ref board) => println!("{board}"),
        Game::Black(ref board) => println!("{board}"),
    }

    // for _ in 0..10 {
    loop {
        game = match game {
            Game::White(board) => {
                let moves = board.team_legal_moves();
                if moves.is_empty() {
                    println!("{:?} has no legal moves, GG!", board.team());
                    break;
                }
                let user_move = user_move(&board);
                match user_move {
                    Ok(legal_move) => {
                        let board = board.apply_move(&legal_move);
                        println!("{board}");
                        Game::Black(board)
                    }
                    Err(e) => {
                        println!("[ERROR] {e}");
                        println!("Example of move syntax are: Ng1-f3 or Bb5xc6");
                        Game::White(board)
                    }
                }
            }
            Game::Black(board) => {
                let moves = board.team_legal_moves();
                if moves.is_empty() {
                    println!("{:?} has no legal moves, GG!", board.team());
                    break;
                }
                let user_move = user_move(&board);
                match user_move {
                    Ok(legal_move) => {
                        let board = board.apply_move(&legal_move);
                        println!("{board}");
                        Game::White(board)
                    }
                    Err(e) => {
                        println!("[ERROR] {e}");
                        println!("Example of move syntax are: Ng1-f3 or Bb5xc6");
                        Game::Black(board)
                    }
                }
            }
        }
    }
}
