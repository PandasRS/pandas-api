// modules/pandas/controller.rs

use rocket::{State, serde::json::Json};
use mongodb::Database;
use rocket_okapi::openapi;

use crate::modules::pandas::dto::{CreatePandaDto, UpdatePandaDto};
use crate::modules::pandas::repository::MongoPandaRepository;
use crate::modules::pandas::service;
use crate::modules::pandas::schema::Panda;

#[openapi(tag = "Pandas")]
#[post("/pandas", format = "json", data = "<panda_dto>")]
pub async fn create_panda(panda_dto: Json<CreatePandaDto>, db: &State<Database>) -> Json<Panda> {
    let mut repo = MongoPandaRepository::new(db.inner().clone());
    service::create_panda(&mut repo, panda_dto.into_inner()).await
}

#[openapi(tag = "Pandas")]
#[get("/pandas")]
pub async fn get_pandas(db: &State<Database>) -> Json<Vec<Panda>> {
    let mut repo = MongoPandaRepository::new(db.inner().clone());
    service::get_pandas(&mut repo).await
}

#[openapi(tag = "Pandas")]
#[get("/pandas/<id>")]
pub async fn get_panda(id: String, db: &State<Database>) -> Option<Json<Panda>> {
    let mut repo = MongoPandaRepository::new(db.inner().clone());
    service::get_panda(&mut repo, &id).await
}

#[openapi(tag = "Pandas")]
#[put("/pandas/<id>", format = "json", data = "<panda_dto>")]
pub async fn update_panda(id: String, panda_dto: Json<UpdatePandaDto>, db: &State<Database>) -> Option<Json<Panda>> {
    let mut repo = MongoPandaRepository::new(db.inner().clone());
    service::update_panda(&mut repo, &id, panda_dto.into_inner()).await
}

#[openapi(tag = "Pandas")]
#[delete("/pandas/<id>")]
pub async fn delete_panda(id: String, db: &State<Database>) -> Json<bool> {
    let mut repo = MongoPandaRepository::new(db.inner().clone());
    Json(service::delete_panda(&mut repo, &id).await)
}
