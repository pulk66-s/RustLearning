use std::fmt;

pub struct Board {
    pub board: Vec<String>,
}

impl Board {
    pub fn new() -> Board {
        return Board {
            board: vec![
                String::from("rnbqkbnr"),
                String::from("pppppppp"),
                String::from("--------"),
                String::from("--------"),
                String::from("--------"),
                String::from("--------"),
                String::from("PPPPPPPP"),
                String::from("RNBQKBNR"),
            ],
        };
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut res: String = String::new();
        res += "  | 1 2 3 4 5 6 7 8\n";
        res += "--|----------------\n";
        let mut line_index: u8 = 'H' as u8;
        for row in self.board.iter() {
            res += &format!("{} |", line_index as char);
            line_index -= 1;
            for c in row.chars() {
                res += &format!(" {}", c);
            }
            res += "\n";
        }
        return write!(f, "{}", res);
    }
}

pub fn get_piece(board: &Board, c: usize, l: char) -> char {
    let x: usize = c - 1;
    let y: usize = 7 - (l as usize - 'A' as usize);
    let line: &String = &board.board[y];
    let res: char = line.as_bytes()[x] as char;
    return res;
}

pub fn set_piece(board: &mut Board, c: usize, l: char, p: char) {
    let x: usize = c - 1;
    let y: usize = 7 - (l as usize - 'A' as usize);
    board.board[y].replace_range(x..x + 1, &format!("{}", p))
}

fn pawn_moves(board: &Board, l: char, c: usize) -> Vec<String> {
    let mut res: Vec<String> = vec![];
    if l < 'H' {
        let front: char = (l as u8 + 1) as char;
        let front_piece: char = get_piece(board, c, front);
        if front_piece == '-' {
            res.push(format!("{}{}", front, c));
        }
        if c > 1 {
            let lpiece: char = get_piece(board, c - 1, front);
            if lpiece != '-' && lpiece.is_lowercase() {
                res.push(format!("{}{}", front, c - 1));
            }
        }
        if c < 8 {
            let rpiece: char = get_piece(board, c + 1, front);
            if rpiece != '-' && rpiece.is_lowercase() {
                res.push(format!("{}{}", front, c + 1));
            }
        }
    }
    return res;
}

fn pawn_moves_enemy(board: &Board, l: char, c: usize) -> Vec<String> {
    let mut res: Vec<String> = vec![];
    if l > 'A' {
        let front: char = (l as u8 - 1) as char;
        let front_piece: char = get_piece(board, c, front);
        if front_piece == '-' {
            res.push(format!("{}{}", front, c));
        }
        if c > 1 {
            let lpiece: char = get_piece(board, c - 1, front);
            if lpiece != '-' && lpiece.is_uppercase() {
                res.push(format!("{}{}", front, c - 1));
            }
        }
        if c < 8 {
            let rpiece: char = get_piece(board, c + 1, front);
            if rpiece != '-' && rpiece.is_uppercase() {
                res.push(format!("{}{}", front, c + 1));
            }
        }
    }
    return res;
}

fn rook_moves(board: &Board, l: char, c: usize, player: usize) -> Vec<String> {
    let mut res: Vec<String> = vec![];
    let mut f: fn(char) -> bool = match player {
        0 => char::is_lowercase,
        _ => char::is_uppercase,
    };
    for i in c + 1..9 {
        match get_piece(board, i, l) {
            '-' => res.push(format!("{}{}", l, i)),
            x => {
                if f(x) {
                    res.push(format!("{}{}", l, i));
                }
                break;
            }
        }
    }
    for i in (1..c).rev() {
        println!("i {} l {}", i, l);
        match get_piece(board, i, l) {
            '-' => res.push(format!("{}{}", l, i)),
            x => {
                if f(x) {
                    res.push(format!("{}{}", l, i));
                }
                break;
            }
        }
    }
    for i in ('A'..l).rev() {
        match get_piece(board, c, i) {
            '-' => res.push(format!("{}{}", i, c)),
            x => {
                if f(x) {
                    res.push(format!("{}{}", i, c));
                }
                break;
            }
        }
    }
    for i in (l as u8 + 1) as char..('H' as u8 + 1) as char {
        match get_piece(board, c, i) {
            '-' => res.push(format!("{}{}", i, c)),
            x => {
                if f(x) {
                    res.push(format!("{}{}", i, c));
                }
                break;
            }
        }
    }
    return res;
}

fn bishop_moves(board: &Board, l: char, c: usize, player: usize) -> Vec<String> {
    let mut res: Vec<String> = vec![];
    let mut y: u8 = 1;
    let f: fn(char) -> bool = match player {
        0 => char::is_lowercase,
        _ => char::is_uppercase,
    };
    for i in c + 1..9 {
        let nl: char = (l as u8 + y) as char;
        let nc: usize = i;
        match get_piece(board, nc, nl) {
            '-' => res.push(format!("{}{}", nl, nc)),
            x => {
                if f(x) {
                    res.push(format!("{}{}", nl, nc));
                }
                break;
            }
        }
        y += 1;
    }
    return res;
}

fn knight_moves(board: &Board, l: char, c: usize, player: usize) -> Vec<String> {
    let mut res: Vec<String> = vec![];
    let f: fn(char) -> bool = match player {
        0 => char::is_lowercase,
        _ => char::is_uppercase,
    };
    println!("c {} l {}", c, l);
    if c > 1 && l < 'G' {
        let piece: char = get_piece(board, c - 1, (l as u8 + 2) as char);
        if piece == '-' || f(piece) {
            res.push(format!("{}{}", (l as u8 + 2) as char, c - 1));
        }
    }
    if c > 2 && l < 'H' {
        let piece: char = get_piece(board, c - 2, (l as u8 + 1) as char);
        if piece == '-' || f(piece) {
            res.push(format!("{}{}", (l as u8 + 1) as char, c - 2));
        }
    }
    if c > 1 && l > 'B' {
        let piece: char = get_piece(board, c - 1, (l as u8 - 2) as char);
        if piece == '-' || f(piece) {
            res.push(format!("{}{}", (l as u8 - 2) as char, c - 1));
        }
    }
    if c > 2 && l > 'A' {
        let piece: char = get_piece(board, c - 2, (l as u8 - 1) as char);
        if piece == '-' || f(piece) {
            res.push(format!("{}{}", (l as u8 - 1) as char, c - 2));
        }
    }
    if c < 8 && l < 'G' {
        let piece: char = get_piece(board, c + 1, (l as u8 + 2) as char);
        if piece == '-' || f(piece) {
            res.push(format!("{}{}", (l as u8 + 2) as char, c + 1));
        }
    }
    if c < 7 && l < 'H' {
        let piece: char = get_piece(board, c + 2, (l as u8 + 1) as char);
        if piece == '-' || f(piece) {
            res.push(format!("{}{}", (l as u8 + 1) as char, c + 2));
        }
    }
    if c < 7 && l > 'A' {
        let piece: char = get_piece(board, c + 2, (l as u8 - 1) as char);
        if piece == '-' || f(piece) {
            res.push(format!("{}{}", (l as u8 - 1) as char, c + 2));
        }
    }
    if c < 8 && l > 'B' {
        let piece: char = get_piece(board, c + 1, (l as u8 - 2) as char);
        if piece == '-' || f(piece) {
            res.push(format!("{}{}", (l as u8 - 2) as char, c + 1));
        }
    }
    return res;
}

fn king_moves(board: &Board, l: char, c: usize, player: usize) -> Vec<String> {
    let mut res: Vec<String> = vec![];
    let f: fn(char) -> bool = match player {
        0 => char::is_lowercase,
        _ => char::is_uppercase,
    };
    if c > 1 {
        if l < 'H' {
            let piece: char = get_piece(board, c - 1, (l as u8 + 1) as char);
            if piece == '-' || f(piece) {
                res.push(format!("{}{}", (l as u8 + 1) as char, c - 1));
            }
        }
        if l > 'A' {
            let piece: char = get_piece(board, c - 1, (l as u8 - 1) as char);
            if piece == '-' || f(piece) {
                res.push(format!("{}{}", (l as u8 - 1) as char, c - 1));
            }
        }
        let piece: char = get_piece(board, c - 1, l);
        if piece == '-' || f(piece) {
            res.push(format!("{}{}", l, c - 1));
        }
    }
    if c < 8 {
        if l < 'H' {
            let piece: char = get_piece(board, c + 1, (l as u8 + 1) as char);
            if piece == '-' || f(piece) {
                res.push(format!("{}{}", (l as u8 + 1) as char, c + 1));
            }
        }
        if l > 'A' {
            let piece: char = get_piece(board, c + 1, (l as u8 - 1) as char);
            if piece == '-' || f(piece) {
                res.push(format!("{}{}", (l as u8 - 1) as char, c + 1));
            }
        }
        let piece: char = get_piece(board, c + 1, l);
        if piece == '-' || f(piece) {
            res.push(format!("{}{}", l, c + 1));
        }
    }
    if l < 'H' {
        let piece: char = get_piece(board, c, (l as u8 + 1) as char);
        if piece == '-' || f(piece) {
            res.push(format!("{}{}", (l as u8 + 1) as char, c));
        }
    }
    if l > 'A' {
        let piece: char = get_piece(board, c, (l as u8 - 1) as char);
        if piece == '-' || f(piece) {
            res.push(format!("{}{}", (l as u8 - 1) as char, c));
        }
    }
    return res;
}

fn queen_moves(board: &Board, l: char, c: usize, player: usize) -> Vec<String> {
    let mut res: Vec<String> = vec![];
    let mut bishop_moves: Vec<String> = bishop_moves(board, l, c, player);
    let mut rook_moves: Vec<String> = rook_moves(board, l, c, player);
    res.append(&mut bishop_moves);
    res.append(&mut rook_moves);
    return res;
}

pub fn get_possible_moves(board: &Board, piece: &String) -> Vec<String> {
    let l: char = piece.as_bytes()[0] as char;
    let c: usize = piece.as_bytes()[1] as usize - '0' as usize;
    return match get_piece(board, c, l) {
        'P' => pawn_moves(board, l, c),
        'p' => pawn_moves_enemy(board, l, c),
        'R' => rook_moves(board, l, c, 0),
        'r' => rook_moves(board, l, c, 1),
        'B' => bishop_moves(board, l, c, 0),
        'b' => bishop_moves(board, l, c, 1),
        'N' => knight_moves(board, l, c, 0),
        'n' => knight_moves(board, l, c, 1),
        'K' => king_moves(board, l, c, 0),
        'k' => king_moves(board, l, c, 1),
        'Q' => queen_moves(board, l, c, 0),
        'q' => queen_moves(board, l, c, 1),
        _ => vec![],
    };
}

pub fn move_piece(board: &mut Board, from: &String, to: &String) {
    let pl: char = from.as_bytes()[0] as char;
    let pc: usize = from.as_bytes()[1] as usize - '0' as usize;
    let tl: char = to.as_bytes()[0] as char;
    let tc: usize = to.as_bytes()[1] as usize - '0' as usize;
    let curr: char = get_piece(board, pc, pl);
    set_piece(board, pc, pl, '-');
    set_piece(board, tc, tl, curr);
}
