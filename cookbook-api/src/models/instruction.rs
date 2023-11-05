use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, FromRow, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct InstructionModel {
    pub id: Uuid,
    pub recipe_id: Uuid,
    pub instruction_text: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateInstructionSchema {
    pub recipe_id: Uuid,
    pub instruction_text: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateInstructionSchema {
    pub instruction_text: String
}