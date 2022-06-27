#[path = "./board.rs"]
mod board;

use crate::board::{get_piece, Board};
use std::io::{stdin, stdout, Write};

fn parse_buf(buf: String) -> (char, usize) {
    return (
        buf.as_bytes()[0] as char,
        buf.as_bytes()[1] as usize - '0' as usize,
    );
}

fn check_line(c: char) -> bool {
    return match c {
        x if ('A'..'I').contains(&x) => true,
        _ => {
            println!("false line");
            false
        }
    };
}

fn check_col(l: usize) -> bool {
    return match l {
        x if (0..9).contains(&x) => true,
        _ => {
            println!("false col");
            false
        }
    };
}

fn match_player_one(board: &Board, l: char, c: usize) -> Option<String> {
    return match check_line(l) && check_col(c) {
        true => match l {
            x if "PRNBQK".contains(&&String::from(format!("{}", get_piece(board, c, x)))[..]) => {
                Some(String::from(format!("{}{}", l, c)))
            }
            _ => None,
        },
        false => None,
    };
}

fn match_player_two(board: &Board, l: char, c: usize) -> Option<String> {
    return match check_line(l) && check_col(c) {
        true => match l {
            x if "prnbqk".contains(&&String::from(format!("{}", get_piece(board, c, x)))[..]) => {
                Some(String::from(format!("{}{}", l, c)))
            }
            _ => None,
        },
        false => None,
    };
}

fn checking_input(board: &Board, player: usize, buf: String) -> Option<String> {
    let c: usize;
    let l: char;
    if buf.len() != 3 {
        return None;
    }
    return match player {
        0 => {
            (l, c) = parse_buf(buf);
            match_player_one(board, l, c)
        }
        1 => {
            (l, c) = parse_buf(buf);
            match_player_two(board, l, c)
        }
        _ => None,
    };
}

pub fn get_user_input(board: &Board, player: usize) -> Option<String> {
    let mut buf: String = String::new();
    print!("Select your pawn : ");
    match stdout().flush() {
        Ok(_) => {}
        Err(_) => return None,
    }
    return match stdin().read_line(&mut buf) {
        Err(_) => None,
        Ok(_) => checking_input(board, player, buf),
    };
}

fn check_good_move(buf: String, moves: &Vec<String>) -> Option<String> {
    let trimmed: &str = buf.trim();
    return match moves.contains(&String::from(trimmed)) {
        true => Some(buf),
        false => None,
    };
}

pub fn select_move(moves: &Vec<String>) -> Option<String> {
    let mut buf: String = String::new();
    println!("Possibles moves : {:?}", moves);
    print!("Choose a move : ");
    match stdout().flush() {
        Ok(_) => {}
        Err(_) => return None,
    }
    return match stdin().read_line(&mut buf) {
        Err(_) => None,
        Ok(_) => check_good_move(buf, moves),
    };
}
