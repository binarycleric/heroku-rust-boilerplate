#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;

mod app;
mod models;
mod schema;

use models::*;

use rocket_contrib::templates::Template;

#[derive(Serialize, Deserialize)]
struct WelcomeView {
    pub name: String,
}

#[get("/")]
fn welcome() -> Template {
    let name = Rustacean::get_random_rustacean_name().unwrap();
    let view = WelcomeView { name };

    Template::render("index", &view)
}

fn main() {
    let routes = routes![welcome];
    app::start(routes);
}
