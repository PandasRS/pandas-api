// modules/pandas/service.rs

use rocket::serde::json::Json;
use mongodb::Database;

use crate::modules::pandas::dto::{CreatePandaDto, UpdatePandaDto};
use crate::modules::pandas::repository;
use crate::modules::pandas::schema::Panda;

pub async fn create_panda(dto: CreatePandaDto, db: &Database) -> Json<Panda> {
    let panda = Panda {
        id: None,
        name: dto.name,
        age: dto.age,
    };
    Json(repository::create_panda(panda, db).await)
}

pub async fn get_pandas(db: &Database) -> Json<Vec<Panda>> {
    Json(repository::get_pandas(db).await)
}

pub async fn get_panda(id: &str, db: &Database) -> Option<Json<Panda>> {
    repository::get_panda(id, db).await.map(Json)
}

pub async fn update_panda(id: &str, dto: UpdatePandaDto, db: &Database) -> Option<Json<Panda>> {
    repository::update_panda(id, dto, db).await.map(Json)
}

pub async fn delete_panda(id: &str, db: &Database) -> bool {
    repository::delete_panda(id, db).await
}
