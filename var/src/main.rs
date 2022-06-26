struct Player {
    name: String,
    hp: u32
}

impl Player {
    pub fn new() -> Player {
        return Player {
            name: String::from("Hello"),
            hp: 32
        };
    }
}

fn add(a: u32, b: u32) -> Option<u32>
{
    if a + b % 2 == 0 {
        return None
    }
    return Some(a + b);
}

fn main()
{
    let player = Player::new();
    println!("player name = {0}", player.name);
    println!("player hp = {0}", player.hp);

    let age: i32 = 42;
    println!("{0}", match age {
        x if x < 18 => "Minor",
        _ => "Major"
    });
    'global: loop {
        'inner: for i in 0..10 {
            break 'global;
        }
    }
}
