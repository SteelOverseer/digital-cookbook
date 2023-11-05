use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, FromRow, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct TagModel {
    pub id: Uuid,
    pub name: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateTagSchema {
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateTagSchema {
    pub name: String
}