use super::models::*;
use rocket_contrib::templates::Template;

#[derive(Serialize, Deserialize)]
struct WelcomeViewContext {
    pub title: String,
    pub name: String,
}

#[get("/")]
pub fn welcome() -> Template {
    let title = "Rust Boilerplate".to_string();
    let name = Rustacean::get_random_rustacean_name().unwrap();
    let view = WelcomeViewContext { title, name };

    Template::render("index", &view)
}
