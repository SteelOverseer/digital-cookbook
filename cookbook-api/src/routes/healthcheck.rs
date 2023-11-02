use actix_web::HttpResponse;
use serde_json::json;

pub async fn health_check() -> HttpResponse {
    const MESSAGE: &str = "API is running";

    HttpResponse::Ok().json(json!({"status": "success","message": MESSAGE}))
}