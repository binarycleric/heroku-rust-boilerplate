#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel;
extern crate dotenv;

use diesel::prelude::*;
use diesel::pg::PgConnection;

use dotenv::dotenv;
use std::env;

pub mod schema;

#[derive(Queryable)]
pub struct Rustacean {
    pub id: i32,
    pub name: String,
}

no_arg_sql_function!(random, (), "Represents the sql random() function");

pub fn pg() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

#[get("/")]
fn index() -> String {
    use self::schema::rustaceans::dsl::*;

    let connection = pg();
    let results = rustaceans
        .order(random)
        .limit(1)
        .load::<Rustacean>(&connection)
        .expect("Error loading rustaceans");

    format!("{} says hello from Rust Boilerplate!", results[0].name)
}

fn main() {
    rocket::ignite().mount("/", routes![index]).launch();
}