use actix_web::{web, Responder, HttpResponse};
use serde_json::json;

use crate::models::{InstructionModel, CreateInstructionSchema, UpdateInstructionSchema, AppState};

pub async fn get_instructions_for_recipe(
    path: web::Path<uuid::Uuid>,
    data: web::Data<AppState>,
) -> impl Responder {
    let recipe_id = path.into_inner();
    let result = sqlx::query_as!(
        InstructionModel,
        "SELECT * FROM instructions WHERE recipe_id = $1 ORDER BY step_order",
        recipe_id
    )
    .fetch_all(&data.db)
    .await;

    match result {
        Ok(instructions) => {
            let response = serde_json::json!(instructions);
            return HttpResponse::Ok().json(response);
        }
        Err(_) => {
            let message = format!("Instructions with Recipe ID: {} not found", recipe_id);
            return HttpResponse::NotFound()
                .json(serde_json::json!(message));
        }

    }
}

pub async fn create_instruction(
    body: web::Json<CreateInstructionSchema>,
    data: web::Data<AppState>
) -> impl Responder {
    let result = sqlx::query_as!(
        InstructionModel,
        "INSERT INTO instructions (recipe_id, instruction_text, step_order) VALUES ($1, $2, $3) RETURNING *",
        body.recipe_id,
        body.instruction_text,
        body.step_order
    )
    .fetch_one(&data.db)
    .await;

    match result {
        Ok(instruction) => {
            let response = serde_json::json!(instruction);

            return HttpResponse::Ok().json(response);
        }
        Err(e) => {
            return HttpResponse::InternalServerError().json(serde_json::json!(format!("{:?}", e)))
        }
    }
}

pub async fn edit_instruction(
    path: web::Path<uuid::Uuid>,
    body: web::Json<UpdateInstructionSchema>,
    data: web::Data<AppState>
) -> impl Responder {
    let instruction_id = path.into_inner();
    let result = sqlx::query_as!(
        InstructionModel,
        "SELECT * FROM instructions WHERE id = $1",
        instruction_id
    )
    .fetch_one(&data.db)
    .await;

    if result.is_err() {
        let message = format!("Instruction with ID: {} not found", instruction_id);
        return HttpResponse::NotFound()
                .json(serde_json::json!(message))
    }

    let instruction = result.unwrap();

    let result = sqlx::query_as!(
        InstructionModel,
        "UPDATE instructions SET instruction_text = $1, step_order = $2 WHERE id = $3 RETURNING *",
        body.instruction_text.to_owned().unwrap_or(instruction.instruction_text),
        body.step_order.to_owned().unwrap_or(instruction.step_order),
        instruction_id
    )
    .fetch_one(&data.db)
    .await;

    match result {
        Ok(instruction) => {
            let response = serde_json::json!(instruction);

            return HttpResponse::Ok().json(response);
        }
        Err(err) => {
            let message = format!("Error: {:?}", err);
            return HttpResponse::InternalServerError()
                .json(serde_json::json!(message));

        }
    }
}

pub async fn delete_instruction(
    path: web::Path<uuid::Uuid>,
    data: web::Data<AppState>
) -> impl Responder {
    let instruction_id = path.into_inner();
    let rows_affected = sqlx::query!(
        "DELETE FROM instructions WHERE id = $1",
        instruction_id
    )
    .execute(&data.db)
    .await
    .unwrap()
    .rows_affected();

    if rows_affected == 0 {
        let message = format!("Instruction with ID: {} not found", instruction_id);
        return HttpResponse::NotFound().json(json!(message));
    }

    HttpResponse::NoContent().finish()
}