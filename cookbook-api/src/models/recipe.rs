use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, FromRow, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct RecipeModel {
    pub id: Uuid,
    pub category_id: Uuid,
    pub name: String,
    pub notes: Option<String>,
    pub is_favorite: bool
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateRecipeSchema {
    pub category_id: Uuid,
    pub name: String,
    pub notes: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateRecipeSchema {
    pub category_id: Option<Uuid>,
    pub name: Option<String>,
    pub notes: Option<String>,
    pub is_favorite: Option<bool>
}