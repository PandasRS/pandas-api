// database/mod.rs

use mongodb::{Client, Database, options::ClientOptions};
use crate::config::AppConfig;

pub async fn establish_mongo_connection(config: &AppConfig) -> Database {
    let client_options = ClientOptions::parse(&config.mongo_url).await.unwrap();
    let client = Client::with_options(client_options).unwrap();
    client.database("pandas")
}