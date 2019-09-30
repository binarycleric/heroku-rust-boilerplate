use super::models::*;
use super::views::WelcomeView;
use rocket_contrib::json::Json;
use rocket_contrib::templates::Template;

pub fn get_routes() -> std::vec::Vec<rocket::Route> {
    routes![welcome, get_random_rustacean]
}

#[get("/")]
fn welcome() -> Template {
    let title = "Rust Boilerplate".to_string();
    let random_rustacean = Rustacean::get_random_rustacean().unwrap();
    let name = random_rustacean.name;
    let view = WelcomeView { title, name };

    Template::render(view.template(), &view)
}

#[get("/random_rustacean.json")]
fn get_random_rustacean() -> Json<Rustacean> {
    let random_rustacean = Rustacean::get_random_rustacean().unwrap();
    Json(random_rustacean)
}
