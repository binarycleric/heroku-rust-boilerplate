#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;
extern crate dotenv;

mod app;
mod models;
mod schema;

use models::*;

#[get("/")]
fn welcome() -> String {

    match Rustacean::get_random_rustacean_name() {
        Some(name) => {
            format!("{} says hello from Rust Boilerplate!", name)
        },
        None => {
            format!("Did you forget to run database migrations?")
        }
    }
}

fn main() {
    let routes = routes![welcome];
    app::start(routes);
}
