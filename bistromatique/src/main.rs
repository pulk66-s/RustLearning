mod calc;

fn get_input() -> Option<String>
{
    let mut input: String = String::new();
    let res = std::io::stdin().read_line(&mut input);
    if res.is_err() {
        println!("input Error");
        return None;
    }
    return Some(input);
}

fn print_res(res: Option<i128>)
{
    match res {
        None => println!("Error"),
        Some(x) => println!("Res: {0}", x)
    }
}

fn parse_calc(j: &str, nb1: &str, nb2: &str)
{
    match j {
        "+" => print_res(calc::add(nb1, nb2)),
        "-" => print_res(calc::sub(nb1, nb2)),
        "*" => print_res(calc::mult(nb1, nb2)),
        _ => print_res(None)
    }
}

fn main()
{
    loop {
        let user_input: String;
        match get_input() {
            None => break,
            Some(x) => user_input = x
        };
        let words: Vec<&str> = user_input.trim().split(" ").collect::<Vec<&str>>();
        let words_len: usize = words.len();
        for (i, j) in words.iter().enumerate() {
            if i > 0 && i < words_len - 1 {
                parse_calc(j, words[i - 1], words[i + 1]);
            }
        }
    }
}
