use super::models::*;
use rocket_contrib::templates::Template;

#[derive(Serialize, Deserialize)]
struct WelcomeViewContext {
    pub name: String,
}

#[get("/")]
pub fn welcome() -> Template {
    let name = Rustacean::get_random_rustacean_name().unwrap();
    let view = WelcomeViewContext { name };

    Template::render("index", &view)
}
