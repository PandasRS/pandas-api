// modules/pandas/controller.rs

use rocket::{State, serde::json::Json};
use mongodb::Database;

use rocket_okapi::openapi;

use crate::modules::pandas::dto::{CreatePandaDto, UpdatePandaDto};
use crate::modules::pandas::service;

#[openapi(tag = "Pandas")]
#[post("/pandas", format = "json", data = "<panda_dto>")]
pub async fn create_panda(panda_dto: Json<CreatePandaDto>, db: &State<Database>) -> Json<crate::modules::pandas::schema::Panda> {
    service::create_panda(panda_dto.into_inner(), db).await
}

#[openapi(tag = "Pandas")]
#[get("/pandas")]
pub async fn get_pandas(db: &State<Database>) -> Json<Vec<crate::modules::pandas::schema::Panda>> {
    service::get_pandas(db).await
}

#[openapi(tag = "Pandas")]
#[get("/pandas/<id>")]
pub async fn get_panda(id: String, db: &State<Database>) -> Option<Json<crate::modules::pandas::schema::Panda>> {
    service::get_panda(&id, db).await
}

#[openapi(tag = "Pandas")]
#[put("/pandas/<id>", format = "json", data = "<panda_dto>")]
pub async fn update_panda(id: String, panda_dto: Json<UpdatePandaDto>, db: &State<Database>) -> Option<Json<crate::modules::pandas::schema::Panda>> {
    service::update_panda(&id, panda_dto.into_inner(), db).await
}

#[openapi(tag = "Pandas")]
#[delete("/pandas/<id>")]
pub async fn delete_panda(id: String, db: &State<Database>) -> Json<bool> {
    Json(service::delete_panda(&id, db).await)
}
