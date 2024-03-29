#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;

mod app;
mod models;
mod routes;
mod schema;
mod views;

fn main() {
    app::start();
}
