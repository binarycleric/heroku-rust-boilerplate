#[derive(Serialize, Deserialize)]
pub struct WelcomeView {
    pub title: String,
    pub name: String,
}

impl WelcomeView {
    pub fn template(&self) -> String {
        "index".to_string()
    }
}
