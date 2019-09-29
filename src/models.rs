use diesel::pg::PgConnection;
use diesel::prelude::*;

use dotenv::dotenv;
use std::env;

fn conn() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

no_arg_sql_function!(random, (), "Represents the sql random() function");

#[derive(Queryable, Serialize, Deserialize, Debug, Clone)]
pub struct Rustacean {
    pub id: i32,
    pub name: String,
}

impl Rustacean {
    pub fn get_random_rustacean() -> Option<Rustacean> {
        use super::schema::rustaceans::dsl::*;

        let connection = conn();
        let results = rustaceans
            .order(random)
            .limit(1)
            .load::<Rustacean>(&connection)
            .expect("Error loading rustaceans");

        Some(results[0].clone())
    }

    pub fn get_random_rustacean_name() -> Option<String> {
        use super::schema::rustaceans::dsl::*;

        let connection = conn();
        let results = rustaceans
            .order(random)
            .limit(1)
            .load::<Rustacean>(&connection)
            .expect("Error loading rustaceans");

        Some(results[0].name.clone())
    }
}
