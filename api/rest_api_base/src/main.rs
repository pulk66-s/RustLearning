#[macro_use]
extern crate rocket;

#[path = "routes/user.rs"]
mod user;

#[get("/")]
fn index() -> &'static str {
    return "Hello World";
}

#[launch]
fn rocket() -> _ {
    rocket::build()
    .mount("/", routes![index])
    .mount("/user", routes![
        user::users,
        user::user_by_name,
        user::create_user
    ])
}