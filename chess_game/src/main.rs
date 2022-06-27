mod board;
mod player;

use board::{get_possible_moves, move_piece, Board};
use player::{get_user_input, select_move};

fn main() {
    let mut board: Board = Board::new();
    let mut player_turn: usize = 0;
    println!("{}", board);
    loop {
        let mut input: String;
        let mut possibles_move: Vec<String>;
        loop {
            loop {
                match get_user_input(&board, player_turn) {
                    Some(x) => {
                        input = x;
                        break;
                    }
                    _ => println!("Error occur"),
                }
            }
            possibles_move = get_possible_moves(&board, &input);
            if possibles_move.len() != 0 {
                break;
            } else {
                println!("No moves possible");
            }
        }
        let choose_move: String;
        loop {
            match select_move(&possibles_move) {
                Some(x) => {
                    choose_move = x;
                    break;
                }
                _ => println!("Error occur"),
            }
        }
        move_piece(&mut board, &input, &choose_move);
        player_turn = (player_turn + 1) % 2;
        println!("{}", board);
    }
}
