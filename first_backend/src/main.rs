#[macro_use]
extern crate rocket;

#[path = "routes/user.rs"]
mod user;

#[launch]
fn launch() -> _ {
    rocket::build()
    .mount("/user", routes![
        user::create_user,
        user::ping_user
    ])
}