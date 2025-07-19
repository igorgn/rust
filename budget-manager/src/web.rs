use rocket::{Build, Rocket};

#[get("/")]
pub fn hello() -> &'static str {
    "Hello, world!"
}

pub fn build_rocket() -> Rocket<Build> {
    rocket::build().mount("/", routes![hello])
}
