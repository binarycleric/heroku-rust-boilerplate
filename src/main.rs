#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;

mod models;
mod schema;
mod app;
mod routes;

fn main() {
    let routes = routes![
        routes::welcome
    ];

    app::start(routes);
}
