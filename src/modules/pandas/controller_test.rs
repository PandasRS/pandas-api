#[cfg(test)]
mod tests {
    use reqwest;
    use std::sync::Once;

    static INIT: Once = Once::new();

    async fn start_app() {
      INIT.call_once(|| {
        let rocket = crate::rocket();
        tokio::spawn(async move {
          rocket.await.launch().await.unwrap();
        });
      });
    }

    #[tokio::test]
    async fn test_create_panda_endpoint() {
      start_app().await;

      let client = reqwest::Client::new();
      let res = client.post("http://localhost:8000/pandas")
        .header("Content-Type", "application/json")
        .header("Accept", "application/json")
        .body(r#"{"name": "Panda", "age": 10}"#)
        .send()
        .await
        .unwrap();

      assert_eq!(res.status(), reqwest::StatusCode::OK);
    }

}
