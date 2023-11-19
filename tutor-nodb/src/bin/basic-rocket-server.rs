#[macro_use]
extern crate rocket;
use rocket::serde::{json::Json, Serialize};

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct Text<'r> {
    status: &'r str,
}

#[get("/health")]
fn health_check_handler() -> Json<Text<'static>> {
    Json(Text {
        status: "Hello Rocket. EzyTutors is alive and kicking",
    })
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![health_check_handler])
}
