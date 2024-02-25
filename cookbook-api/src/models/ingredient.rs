use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, FromRow, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct IngredientModel {
    pub id: Uuid,
    pub recipe_id: Uuid,
    pub ingredient_text: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateIngredientSchema {
    pub recipe_id: Uuid,
    pub ingredient_text: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateIngredientSchema {
    pub ingredient_text: String
}