use actix_web::{web, Responder, HttpResponse};
use serde_json::json;

use crate::models::{TagModel, CreateTagSchema, UpdateTagSchema, AppState};

pub async fn get_tags(data: web::Data<AppState>) -> impl Responder {
    let result = sqlx::query_as!(
        TagModel,
        "SELECT * FROM tags ORDER BY id"
    )
    .fetch_all(&data.db)
    .await;

    if result.is_err() {
        let message = "There was an error fetching all tags";
        return HttpResponse::InternalServerError()
            .json(json!({
                "status": "error",
                "message": message
            }))
    }

    let tags = result.unwrap();

    let response = serde_json::json!({
        "status": "success",
        "results": tags.len(),
        "tags": tags
    });

    HttpResponse::Ok().json(response)
}

pub async fn create_tag(
    body: web::Json<CreateTagSchema>,
    data: web::Data<AppState>
) -> impl Responder {
    let result = sqlx::query_as!(
        TagModel,
        "INSERT INTO tags (name) VALUES ($1) RETURNING *",
        body.name
    )
    .fetch_one(&data.db)
    .await;

    match result {
        Ok(tag) => {
            let response = serde_json::json!({
                "status": "success",
                "tag": tag
            });

            return HttpResponse::Ok().json(response)
        }
        Err(e) => {
            if e.to_string()
            .contains("duplicate key value violates unique constraint")
        {
            return HttpResponse::BadRequest()
            .json(serde_json::json!({"status": "fail","message": "Tag with that name already exists"}));
        }

        return HttpResponse::InternalServerError()
            .json(serde_json::json!({"status": "error","message": format!("{:?}", e)}));

        }
    }
}

pub async fn edit_tag(
    path: web::Path<uuid::Uuid>,
    body: web::Json<UpdateTagSchema>,
    data: web::Data<AppState>
) -> impl Responder {
    let tag_id = path.into_inner();
    let result = sqlx::query_as!(
        TagModel,
        "SELECT * FROM tags WHERE id = $1",
        tag_id
    )
    .fetch_one(&data.db)
    .await;

    if result.is_err() {
        let message = format!("Tag with ID: {} not found", tag_id);
        return HttpResponse::NotFound()
            .json(serde_json::json!({"status": "fail","message": message}));
    }

    let result = sqlx::query_as!(
        TagModel,
        "UPDATE tags SET name = $1 WHERE id = $2 RETURNING *",
        body.name,
        tag_id
    )
    .fetch_one(&data.db)
    .await;

    match result {
        Ok(tag) => {
            let response = serde_json::json!({
                "status": "success",
                "tag": tag
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

pub async fn delete_tag(
    path: web::Path<uuid::Uuid>,
    data: web::Data<AppState>
) -> impl Responder {
    let tag_id = path.into_inner();
    let rows_affected = sqlx::query!(
        "DELETE FROM tags WHERE id = $1",
        tag_id
    )
    .execute(&data.db)
    .await
    .unwrap()
    .rows_affected();

    if rows_affected == 0 {
        let message = format!("Tag with ID: {} not found", tag_id);
        return HttpResponse::NotFound().json(json!({
            "status": "fail",
            "message": message
        }));
    }

    HttpResponse::NoContent().finish()
}