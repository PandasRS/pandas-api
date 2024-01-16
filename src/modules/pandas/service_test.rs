#[cfg(test)]
mod tests {
    use crate::modules::pandas::dto::{CreatePandaDto, UpdatePandaDto};
    use crate::modules::pandas::service::*;
    use crate::modules::pandas::repository_mock::MockPandaRepository;

    #[tokio::test]
    async fn test_create_panda_service() {
        let mut mock_repo = MockPandaRepository::new();
        let dto = CreatePandaDto {
            name: "New Panda".to_string(),
            age: 5,
        };

        let result = create_panda(&mut mock_repo, dto).await.into_inner();
        assert_eq!(result.name, "New Panda");
        assert_eq!(result.age, 5);
        assert!(result.id.is_some());
    }

    #[tokio::test]
    async fn test_get_pandas_service() {
        let mut mock_repo = MockPandaRepository::new();
        mock_repo.insert_mock_panda("1", "Panda One", 3);
        mock_repo.insert_mock_panda("2", "Panda Two", 4);

        let result = get_pandas(&mut mock_repo).await.into_inner();
        assert_eq!(result.len(), 2);
    }

    #[tokio::test]
    async fn test_get_panda_service() {
      let mut mock_repo = MockPandaRepository::new();
      mock_repo.insert_mock_panda(&"123".to_string(), "Single Panda", 5);

      let result = get_panda(&mut mock_repo, &"123".to_string()).await;
      assert!(result.is_some());
      let panda = result.unwrap().into_inner();
      assert_eq!(panda.name, "Single Panda");
      assert_eq!(panda.age, 5);
    }

    #[tokio::test]
    async fn test_update_panda_service() {
        let mut mock_repo = MockPandaRepository::new();
        mock_repo.insert_mock_panda("update_id", "Old Panda", 7);

        let update_dto = UpdatePandaDto {
            name: Some("Updated Panda".to_string()),
            age: Some(8),
        };

        let result = update_panda(&mut mock_repo, "update_id", update_dto).await;
        assert!(result.is_some());
        let updated_panda = result.unwrap().into_inner();
        assert_eq!(updated_panda.name, "Updated Panda");
        assert_eq!(updated_panda.age, 8);
    }

    #[tokio::test]
    async fn test_delete_panda_service() {
        let mut mock_repo = MockPandaRepository::new();
        mock_repo.insert_mock_panda("delete_id", "Delete Panda", 6);

        let result = delete_panda(&mut mock_repo, "delete_id").await;
        assert!(result);
    }
}
