use super::models::*;
use rocket_contrib::json::Json;
use rocket_contrib::templates::Template;

#[derive(Serialize, Deserialize)]
struct WelcomeViewContext {
    pub title: String,
    pub name: String,
}

#[get("/")]
pub fn welcome() -> Template {
    let title = "Rust Boilerplate".to_string();
    let random_rustacean = Rustacean::get_random_rustacean().unwrap();
    let name = random_rustacean.name;
    let view = WelcomeViewContext { title, name };

    Template::render("index", &view)
}

#[get("/random_rustacean.json")]
pub fn get_random_rustacean() -> Json<Rustacean> {
    let random_rustacean = Rustacean::get_random_rustacean().unwrap();
    Json(random_rustacean)
}
