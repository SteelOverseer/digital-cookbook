use crate::{
    model::CategoryModel,
    schema::{CreateCategorySchema, FilterOptions, UpdateCategorySchema},
    AppState,
};
use actix_web::{delete, get, patch, post, web, HttpResponse, Responder};
use chrono::prelude::*;
use serde_json::json;

#[get("/healthchecker")]
async fn health_checker_handler() -> impl Responder {
    const MESSAGE: &str = "Build Simple CRUD API with Rust, SQLX, Postgres,and Actix Web";

    HttpResponse::Ok().json(json!({"status": "success","message": MESSAGE}))
}

#[get("/categories")]
pub async fn category_list_handler(
    opts: web::Query<FilterOptions>,
    data: web::Data<AppState>,
) -> impl Responder {
    let limit = opts.limit.unwrap_or(10);
    let offset = (opts.page.unwrap_or(1) - 1) * limit;

    let query_result = sqlx::query_as!(
        CategoryModel,
        "SELECT * FROM categories ORDER by id LIMIT $1 OFFSET $2",
        limit as i32,
        offset as i32
    )
    .fetch_all(&data.db)
    .await;

    if query_result.is_err() {
        let message = "Something bad happened while fetching all categories";
        return HttpResponse::InternalServerError()
            .json(json!({"status": "error","message": message}));
    }

    let notes = query_result.unwrap();

    let json_response = serde_json::json!({
        "status": "success",
        "results": notes.len(),
        "notes": notes
    });
    HttpResponse::Ok().json(json_response)
}

#[post("/category")]
async fn create_category_handler(
    body: web::Json<CreateCategorySchema>,
    data: web::Data<AppState>,
) -> impl Responder {
    let query_result = sqlx::query_as!(
        CategoryModel,
        "INSERT INTO categories (name) VALUES ($1) RETURNING *",
        body.name
    )
    .fetch_one(&data.db)
    .await;

    match query_result {
        Ok(note) => {
            let note_response = serde_json::json!({"status": "success","data": serde_json::json!({
                "note": note
            })});

            return HttpResponse::Ok().json(note_response);
        }
        Err(e) => {
            if e.to_string()
                .contains("duplicate key value violates unique constraint")
            {
                return HttpResponse::BadRequest()
                .json(serde_json::json!({"status": "fail","message": "Category with that name already exists"}));
            }

            return HttpResponse::InternalServerError()
                .json(serde_json::json!({"status": "error","message": format!("{:?}", e)}));
        }
    }
}

#[get("/category/{id}")]
async fn get_category_handler(
    path: web::Path<uuid::Uuid>,
    data: web::Data<AppState>,
) -> impl Responder {
    let category_id = path.into_inner();
    let query_result = sqlx::query_as!(CategoryModel, "SELECT * FROM categories WHERE id = $1", category_id)
        .fetch_one(&data.db)
        .await;

    match query_result {
        Ok(category) => {
            let category_response = serde_json::json!({"status": "success","data": serde_json::json!({
                "category": category
            })});

            return HttpResponse::Ok().json(category_response);
        }
        Err(_) => {
            let message = format!("Category with ID: {} not found", category_id);
            return HttpResponse::NotFound()
                .json(serde_json::json!({"status": "fail","message": message}));
        }
    }
}

#[patch("/category/{id}")]
async fn edit_category_handler(
    path: web::Path<uuid::Uuid>,
    body: web::Json<UpdateCategorySchema>,
    data: web::Data<AppState>,
) -> impl Responder {
    let category_id = path.into_inner();
    let query_result = sqlx::query_as!(CategoryModel, "SELECT * FROM categories WHERE id = $1", category_id)
        .fetch_one(&data.db)
        .await;

    if query_result.is_err() {
        let message = format!("Category with ID: {} not found", category_id);
        return HttpResponse::NotFound()
            .json(serde_json::json!({"status": "fail","message": message}));
    }

    let query_result = sqlx::query_as!(
        CategoryModel,
        "UPDATE categories SET name = $1 RETURNING *",
        body.name
    )
    .fetch_one(&data.db)
    .await
    ;

    match query_result {
        Ok(category) => {
            let category_response = serde_json::json!({"status": "success","data": serde_json::json!({
                "category": category
            })});

            return HttpResponse::Ok().json(category_response);
        }
        Err(err) => {
            let message = format!("Error: {:?}", err);
            return HttpResponse::InternalServerError()
                .json(serde_json::json!({"status": "error","message": message}));
        }
    }
}

#[delete("/category/{id}")]
async fn delete_category_handler(
    path: web::Path<uuid::Uuid>,
    data: web::Data<AppState>,
) -> impl Responder {
    let category_id = path.into_inner();
    let rows_affected = sqlx::query!("DELETE FROM categories WHERE id = $1", category_id)
        .execute(&data.db)
        .await
        .unwrap()
        .rows_affected();

    if rows_affected == 0 {
        let message = format!("Category with ID: {} not found", category_id);
        return HttpResponse::NotFound().json(json!({"status": "fail","message": message}));
    }

    HttpResponse::NoContent().finish()
}

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/api")
        .service(health_checker_handler)
        .service(category_list_handler)
        .service(create_category_handler)
        .service(get_category_handler)
        .service(edit_category_handler)
        .service(delete_category_handler);

    conf.service(scope);
}


