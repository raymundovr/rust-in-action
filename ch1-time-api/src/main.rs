#![feature(proc_macro_hygiene, decl_macro)]

extern crate serde;
extern crate chrono;
#[macro_use]
extern crate rocket;
extern crate rocket_contrib;

#[macro_use]
extern crate serde_derive;

use chrono::prelude::*;
use rocket_contrib::json::Json;

#[derive(Serialize)]
struct Timestamp {
    time: String,
}

#[get("/index")]
fn index() -> &'static str {
    "Hola mundo!"
}

#[get("/time")]
fn time_now() -> Json<Timestamp> {
    let now: DateTime<Utc> = Utc::now();
    let timestamp = Timestamp {time: now.to_rfc3339()};
    Json(timestamp)
}

fn main() {
    rocket::ignite()
    .mount("/", routes![index, time_now])
    .launch();
}
