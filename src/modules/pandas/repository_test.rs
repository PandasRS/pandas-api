#[cfg(test)]
mod tests {
    use super::*;
    use mongodb::{Client, options::ClientOptions};
    use once_cell::sync::Lazy;

    // Establish a connection to the database for testing
    static DB: Lazy<mongodb::Database> = Lazy::new(|| {
        tokio::runtime::Runtime::new().unwrap().block_on(async {
            let client_options = ClientOptions::parse("your-mongodb-test-uri").await.unwrap();
            let client = Client::with_options(client_options).unwrap();
            client.database("your-test-database")
        })
    });

    #[tokio::test]
    async fn test_create_panda() {
        let db = &DB;
        let test_panda = Panda {
            id: None,
            name: "Test Panda".to_string(),
            age: 5,
        };

        let created_panda = create_panda(test_panda, db).await;
        assert_eq!(created_panda.name, "Test Panda");
        // More assertions and cleanup...
    }

    // Additional tests for get_pandas, get_panda, delete_panda...
}
