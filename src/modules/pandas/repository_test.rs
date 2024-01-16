#[cfg(test)]
mod tests {
    use crate::modules::pandas::dto::UpdatePandaDto;
    use crate::modules::pandas::schema::Panda;
    use crate::modules::pandas::repository::{MongoPandaRepository, PandaRepository};
    use mongodb::{bson::{doc, oid::ObjectId}, Client, options::ClientOptions};

    async fn setup_db() -> mongodb::Database {
        let client_options = ClientOptions::parse("mongodb://localhost:27017").await.unwrap();
        let client = Client::with_options(client_options).unwrap();
        client.database("test")
    }

    async fn insert_test_panda(id: &str, db: &mongodb::Database) {
        let collection = db.collection("pandas");
        let test_panda_doc = doc! {
            "_id": ObjectId::parse_str(id).unwrap(),
            "name": "Test Panda",
            "age": 5
        };

        let _ = collection.insert_one(test_panda_doc, None).await;
    }

    #[tokio::test]
    async fn test_create_panda() {
        let db = setup_db().await;
        let mut repo = MongoPandaRepository::new(db);
        let test_panda = Panda {
            id: None,
            name: "Test Panda".to_string(),
            age: 5,
        };
        let created_panda = repo.create_panda(test_panda).await;
        assert_eq!(created_panda.name, "Test Panda");
    }

    
    #[tokio::test]
    async fn test_get_pandas() {
        let db = setup_db().await;
        insert_test_panda("65a57861ecee3481e654de7e", &db).await;
        insert_test_panda("65a5788d27e2d00a6b8f660f", &db).await;
        let mut repo = MongoPandaRepository::new(db);
        let pandas = repo.get_pandas().await;
        assert!(!pandas.is_empty());
    }

    
    #[tokio::test]
    async fn test_get_panda() {
        let db = setup_db().await;
        insert_test_panda("65a5499c0257cdd737cdbea7", &db).await;
        let mut repo = MongoPandaRepository::new(db);
        let panda = repo.get_panda("65a5499c0257cdd737cdbea7").await;
        assert!(panda.is_some());
        assert_eq!(panda.unwrap().name, "Test Panda");
    }

    
    #[tokio::test]
    async fn test_update_panda() {
        let db = setup_db().await;
        insert_test_panda("65a575aded0c501507eb0832",&db).await;
        let mut repo = MongoPandaRepository::new(db);
        let dto = UpdatePandaDto {
            name: Some("Updated Panda".to_string()),
            age: Some(6),
        };
        let updated_panda = repo.update_panda("65a575aded0c501507eb0832", dto).await;
        assert_eq!(updated_panda.unwrap().name, "Updated Panda");
    }

    
    #[tokio::test]
    async fn test_delete_panda() {
        let db = setup_db().await;
        insert_test_panda("65a5762858a308de24fc0dd1",&db).await;
        let mut repo = MongoPandaRepository::new(db);
        let was_deleted = repo.delete_panda("65a5762858a308de24fc0dd1").await;
        assert!(was_deleted);
    }
}