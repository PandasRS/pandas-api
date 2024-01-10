// config/mod.rs

pub struct AppConfig {
    pub mongo_url: String,
    // Add more configuration options as needed
}

impl AppConfig {
    pub fn new() -> Self {
        AppConfig {
            mongo_url: "mongodb://localhost:27017".to_string(),
            // Initialize other config options here
        }
    }
}
