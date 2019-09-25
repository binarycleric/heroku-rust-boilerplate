use std::env;
use rocket::{Config, Route};

pub fn start(routes: Vec<Route>) {
    let mut config = Config::active().unwrap();
    let port = env::var("PORT")
        .unwrap_or("8000".to_string())
        .parse()
        .unwrap();

    config.set_port(port);

    rocket::custom(config)
        .mount("/", routes)
        .launch();
}
