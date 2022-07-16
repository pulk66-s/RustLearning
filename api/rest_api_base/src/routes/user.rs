use rocket::serde::Deserialize;
use rocket::serde::json::Json;
use std::fmt;

#[get("/")]
pub fn users() -> &'static str {
    return "user list"
}

#[get("/<name>")]
pub fn user_by_name(name: &str) -> String {
    return format!("Hello {}", name);
}

#[derive(Deserialize)]
pub struct User {
    username: Option<String>
}

impl User {
    pub fn new() -> User {
        return User {
            username: None
        };
    }
}

impl fmt::Display for User {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(f, "username : {:?}", self.username);
    }
}


#[post("/", data = "<user>")]
pub fn create_user(user: Json<User>) -> String {
    println!("{}", user.to_string());
    return "test".to_string();
}