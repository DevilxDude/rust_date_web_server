// Import Rocket
#[macro_use]
extern crate rocket;

mod routes;
mod services;

use routes::date::get_current_date;

// Default get route
#[get("/")]
fn say_hello() -> &'static str {
    "Hello, welcome to the api!"
}

// Starting Rocket Server
#[launch]
fn rocket() -> _ {
    rocket::build().mount("/api", routes![say_hello, get_current_date])
}
