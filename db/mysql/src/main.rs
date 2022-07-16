use chrono::prelude::*;
use mysql::prelude::*;
use mysql::*; //For date and time
use std::fmt;

struct User {
    id: usize,
    username: String,
    password: String
}

impl fmt::Display for User {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(f, "id: {}, username: {}, password: {}", self.id, self.username, self.password);
    }
}

fn convert_to_struct(r: (usize, String, String)) -> User {
    return User {
        id: r.0,
        username: r.1,
        password: r.2
    };
}

fn main() {
    let url: &str = "mysql://test:__T3sT1ngçç@localhost:3306/PokeClicker";
    let opts = Opts::from_url(url).expect("Error");
    let pool = Pool::new(opts).unwrap();
    let mut conn = pool.get_conn().unwrap();
    match conn.query_iter("insert into User (username, password) values ('hello3', 'world')") {
        Ok(x) => {
            println!("Query sent");
            println!("{:?}", x);
        },
        Err(x) => {
            println!("Err");
            println!("{}", x);
        }
    };

    let mut res: Vec<User> = vec!();
    match conn.query_iter("select * from User") {
        Ok(x) => {
            println!("Result :");
            x.for_each(|row| {
                let r: (usize, String, String) = from_row(row.unwrap());
                res.push(convert_to_struct(r));
            });
        },
        Err(x) => {
            println!("ERR");
            println!("{}", x);
        }
    };
    println!("Result :");
    for u in res {
        println!("{}", u);
    }
}
