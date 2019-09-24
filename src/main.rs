#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::prelude::*;
use rocket::Config;
use std::env;

pub mod db;
pub mod models;
pub mod schema;

use models::Rustacean;

no_arg_sql_function!(random, (), "Represents the sql random() function");

#[get("/")]
fn index() -> String {
    use self::schema::rustaceans::dsl::*;

    let connection = db::conn();
    let results = rustaceans
        .order(random)
        .limit(1)
        .load::<Rustacean>(&connection)
        .expect("Error loading rustaceans");

    format!("{} says hello from Rust Boilerplate!", results[0].name)
}

fn main() {
    let mut config = Config::active().unwrap();
    let routes = routes![index];
    let port = env::var("PORT")
        .unwrap_or("8000".to_string())
        .parse()
        .unwrap();

    config.set_port(port);

    rocket::custom(config).mount("/", routes).launch();
}
