use actix_web::{web, Responder, HttpResponse};
use serde_json::json;

use crate::models::{IngredientModel, CreateIngredientSchema, UpdateIngredientSchema, AppState};

pub async fn get_ingredients_for_recipe(
    path: web::Path<uuid::Uuid>,
    data: web::Data<AppState>,
) -> impl Responder {
    let recipe_id = path.into_inner();
    let result = sqlx::query_as!(
        IngredientModel,
        "SELECT * FROM ingredients WHERE recipe_id = $1 ORDER BY id",
        recipe_id
    )
    .fetch_all(&data.db)
    .await;

    match result {
        Ok(ingredients) => {
            let response = serde_json::json!(ingredients);
            return HttpResponse::Ok().json(response);
        }
        Err(_) => {
            let message = format!("ingredients with Recipe ID: {} not found", recipe_id);
            return HttpResponse::NotFound()
                .json(serde_json::json!(message));
        }

    }
}

pub async fn create_ingredient(
    body: web::Json<CreateIngredientSchema>,
    data: web::Data<AppState>
) -> impl Responder {
    let result = sqlx::query_as!(
        IngredientModel,
        "INSERT INTO ingredients (recipe_id, ingredient_text) VALUES ($1, $2) RETURNING *",
        body.recipe_id,
        body.ingredient_text
    )
    .fetch_one(&data.db)
    .await;

    match result {
        Ok(ingredient) => {
            let response = serde_json::json!(ingredient);

            return HttpResponse::Ok().json(response);
        }
        Err(e) => {
            return HttpResponse::InternalServerError().json(serde_json::json!(format!("{:?}", e)))
        }
    }
}

pub async fn edit_ingredient(
    path: web::Path<uuid::Uuid>,
    body: web::Json<UpdateIngredientSchema>,
    data: web::Data<AppState>
) -> impl Responder {
    let ingredient_id = path.into_inner();
    let result = sqlx::query_as!(
        IngredientModel,
        "SELECT * FROM ingredients WHERE id = $1",
        ingredient_id
    )
    .fetch_one(&data.db)
    .await;

    if result.is_err() {
        let message = format!("Ingredient with ID: {} not found", ingredient_id);
        return HttpResponse::NotFound()
                .json(serde_json::json!(message))
    }

    let result = sqlx::query_as!(
        IngredientModel,
        "UPDATE ingredients SET ingredient_text = $1 WHERE id = $2 RETURNING *",
        body.ingredient_text.to_owned(),
        ingredient_id
    )
    .fetch_one(&data.db)
    .await;

    match result {
        Ok(ingredient) => {
            let response = serde_json::json!(ingredient);

            return HttpResponse::Ok().json(response);
        }
        Err(err) => {
            let message = format!("Error: {:?}", err);
            return HttpResponse::InternalServerError()
                .json(serde_json::json!(message));

        }
    }
}

pub async fn delete_ingredient(
    path: web::Path<uuid::Uuid>,
    data: web::Data<AppState>
) -> impl Responder {
    let ingredient_id = path.into_inner();
    let rows_affected = sqlx::query!(
        "DELETE FROM ingredients WHERE id = $1",
        ingredient_id
    )
    .execute(&data.db)
    .await
    .unwrap()
    .rows_affected();

    if rows_affected == 0 {
        let message = format!("Ingredient with ID: {} not found", ingredient_id);
        return HttpResponse::NotFound().json(json!(message));
    }

    HttpResponse::NoContent().finish()
}