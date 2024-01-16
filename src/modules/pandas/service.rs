// modules/pandas/service.rs

use rocket::serde::json::Json;
use crate::modules::pandas::dto::{CreatePandaDto, UpdatePandaDto};
use crate::modules::pandas::repository::PandaRepository;
use crate::modules::pandas::schema::Panda;

pub async fn create_panda<R: PandaRepository>(repo: &mut R, dto: CreatePandaDto) -> Json<Panda> {
    let panda = Panda {
        id: None,
        name: dto.name,
        age: dto.age,
    };
    Json(repo.create_panda(panda).await)
}

pub async fn get_pandas<R: PandaRepository>(repo: &mut R) -> Json<Vec<Panda>> {
    Json(repo.get_pandas().await)
}

pub async fn get_panda<R: PandaRepository>(repo: &mut R,id: &String) -> Option<Json<Panda>> {
    repo.get_panda(id).await.map(Json)
}

pub async fn update_panda<R: PandaRepository>(repo: &mut R,id: &str, dto: UpdatePandaDto) -> Option<Json<Panda>> {
    repo.update_panda(id, dto).await.map(Json)
}

pub async fn delete_panda<R: PandaRepository>(repo: &mut R,id: &str) -> bool {
    repo.delete_panda(id).await
}
