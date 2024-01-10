// modules/pandas/dto.rs

use schemars::JsonSchema;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct CreatePandaDto {
    pub name: String,
    pub age: i32,
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct UpdatePandaDto {
    pub name: Option<String>,
    pub age: Option<i32>,
}