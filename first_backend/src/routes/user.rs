use rocket::{form::Form, form::FromForm, serde::Deserialize, serde::Serialize};

#[derive(Serialize, Deserialize)]
pub struct User<'r> {
    username: &'r str,
    password: &'r str
}

#[post("/create", format="application/json", data = "<data>")]
pub fn create_user(data: Json<User<'_>>) -> String {
    println!("create user");
    println!("username {} password {}", data.username, data.password);
    return "Hello".to_string();
}

#[get("/ping")]
pub fn ping_user() -> String {
    println!("Ping user");
    return "Pong".to_string();
}