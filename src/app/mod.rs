use rocket::{Config, Route};
use rocket::config::LoggingLevel;
use rocket_contrib::serve::StaticFiles;
use rocket_contrib::templates::Template;
use std::env;

pub fn start(routes: Vec<Route>) {
    let mut config = Config::active().unwrap();
    let port = env::var("PORT")
        .unwrap_or("8000".to_string())
        .parse()
        .unwrap();

    config.set_log_level(LoggingLevel::Normal);
    config.set_port(port);

    rocket::custom(config)
        .mount("/public", StaticFiles::from("./public"))
        .mount("/", routes)
        .attach(Template::fairing())
        .launch();
}
