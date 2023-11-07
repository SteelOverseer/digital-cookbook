use actix_web::{web, Responder, HttpResponse};
use serde_json::json;

use crate::models::{RecipeModel, CreateRecipeSchema, FilterOptions, UpdateRecipeSchema, AppState};

pub async fn get_recipes(
    opts: web::Query<FilterOptions>,
    data: web::Data<AppState>,
) -> impl Responder {
    let limit = opts.limit.unwrap_or(10);
    let offset = (opts.page.unwrap_or(1) - 1) * limit;

    let result = sqlx::query_as!(
        RecipeModel,
        "SELECT * FROM recipes ORDER BY id LIMIT $1 OFFSET $2",
        limit as i32,
        offset as i32
    )
    .fetch_all(&data.db)
    .await;

    if result.is_err() {
        let message = "There was an error fetching all recipes.";
        return HttpResponse::InternalServerError()
            .json(json!({"status": "error", "message": message}));
    }

    let recipes = result.unwrap();

    let json_response = serde_json::json!({
        "status": "success",
        "results": recipes.len(),
        "recipes": recipes
    });
    HttpResponse::Ok().json(json_response)
}

pub async fn get_recipes_by_category(
    path: web::Path<uuid::Uuid>,
    data: web::Data<AppState>,
) -> impl Responder {
    let category_id = path.into_inner();
    let result = sqlx::query_as!(
        RecipeModel,
        "SELECT * FROM recipes WHERE category_id = $1 ORDER BY id",
        category_id
    )
    .fetch_all(&data.db)
    .await;

    match result {
        Ok(recipes) => {
            let response = serde_json::json!({
                "status": "success",
                "recipes": recipes
            });
            return HttpResponse::Ok().json(response);
        }
        Err(_) => {
            let message = format!("Recipes with Category ID: {} not found", category_id);
            return HttpResponse::NotFound()
                .json(serde_json::json!({"status": "fail","message": message}));
        }

    }
}

pub async fn create_recipe(
    body: web::Json<CreateRecipeSchema>,
    data: web::Data<AppState>,
) -> impl Responder {
    let result = sqlx::query_as!(
        RecipeModel,
        "INSERT INTO recipes (category_id, name, notes, ingredients) VALUES ($1, $2, $3, $4) RETURNING *",
        body.category_id,
        body.name,
        body.notes,
        body.ingredients
    )
    .fetch_one(&data.db)
    .await;

    match result {
        Ok(recipe) => {
            let response = serde_json::json!({
                "status": "success",
                "recipe": recipe
            });
            return HttpResponse::Ok().json(response);
        }
        Err(e) => {
            return HttpResponse::InternalServerError()
            .json(serde_json::json!({"status": "error","message": format!("{:?}", e)}));
        }
    }
}

pub async fn get_recipe(
    path: web::Path<uuid::Uuid>,
    data: web::Data<AppState>
) -> impl Responder {
    let recipe_id = path.into_inner();
    let result = sqlx::query_as!(
        RecipeModel,
        "SELECT * FROM recipes WHERE id = $1",
        recipe_id
    )
    .fetch_one(&data.db)
    .await;

    match result {
        Ok(recipe) => {
            let response = serde_json::json!({
                "status": "success",
                "recipe": recipe
            });

            return HttpResponse::Ok().json(response);
        }
        Err(e) => {
            return HttpResponse::NotFound()
            .json(serde_json::json!({"status": "error","message": format!("{:?}", e)}));
        }
    }
}

pub async fn edit_recipe(
    path: web::Path<uuid::Uuid>,
    body: web::Json<UpdateRecipeSchema>,
    data: web::Data<AppState>,
) -> impl Responder {
    let recipe_id = path.into_inner();
    let result = sqlx::query_as!(
        RecipeModel, 
        "SELECT * FROM recipes WHERE id = $1", 
        recipe_id
    )
    .fetch_one(&data.db)
    .await;

    if result.is_err() {
        let message = format!("Recipe with ID: {} not found", recipe_id);
        return HttpResponse::NotFound()
            .json(serde_json::json!({"status": "fail","message": message}));
    }

    let recipe = result.unwrap();

    let result = sqlx::query_as!(
        RecipeModel,
        "UPDATE recipes SET category_id = $1, name = $2, notes = $3, ingredients = $4 WHERE id = $5 RETURNING *",
        body.category_id.to_owned().unwrap_or(recipe.category_id),
        body.name.to_owned().unwrap_or(recipe.name),
        body.notes.to_owned().unwrap_or(recipe.notes.unwrap()),
        body.ingredients.to_owned().unwrap_or(recipe.ingredients.unwrap()),
        recipe_id
    )
    .fetch_one(&data.db)
    .await;

    match result {
        Ok(recipe) => {
            let response = serde_json::json!({
                "status": "success",
                "recipe": recipe
            });

            return HttpResponse::Ok().json(response);
        }
        Err(err) => {
            let message = format!("Error: {:?}", err);
            return HttpResponse::InternalServerError()
                .json(serde_json::json!({"status": "error","message": message}));
        }
    }
}

pub async fn delete_recipe(
    path: web::Path<uuid::Uuid>,
    data: web::Data<AppState>,
) -> impl Responder {
    let recipe_id = path.into_inner();
    let rows_affected = sqlx::query!("DELETE FROM recipes WHERE id = $1", recipe_id)
        .execute(&data.db)
        .await
        .unwrap()
        .rows_affected();

    if rows_affected == 0 {
        let message = format!("Recipe with ID: {} not found", recipe_id);
        return HttpResponse::NotFound().json(json!({"status": "fail","message": message}));
    }

    HttpResponse::NoContent().finish()
}

pub async fn add_tag(
    recipe_path: web::Path<uuid::Uuid>,
    tag_path: web::Path<uuid::Uuid>,
    data: web::Data<AppState>
) -> impl Responder {
    let recipe_id = recipe_path.into_inner();
    let tag_id = tag_path.into_inner();

    let result = sqlx::query!(
        "INSERT INTO recipe_tag_map (recipe_id, tag_id) VALUES ($1, $2)",
        recipe_id,
        tag_id
    )
    .fetch_one(&data.db)
    .await;

    match result {
        Ok(_) => {
            return HttpResponse::NoContent().finish();
        }
        Err(err) => {
            let message = format!("Error: {:?}", err);
            return HttpResponse::InternalServerError()
                .json(serde_json::json!({"status": "error","message": message}));
        }
    }
}

pub async fn remove_tag(
    recipe_path: web::Path<uuid::Uuid>,
    tag_path: web::Path<uuid::Uuid>,
    data: web::Data<AppState>
) -> impl Responder {
    let recipe_id = recipe_path.into_inner();
    let tag_id = tag_path.into_inner();

    let result = sqlx::query!(
        "DELETE FROM recipe_tag_map WHERE recipe_id = $1 AND tag_id = $2",
        recipe_id,
        tag_id
    )
    .fetch_one(&data.db)
    .await;

    match result {
        Ok(_) => {
            return HttpResponse::NoContent().finish();
        }
        Err(err) => {
            let message = format!("Error: {:?}", err);
            return HttpResponse::InternalServerError()
                .json(serde_json::json!({"status": "error","message": message}));
        }
    }
}