use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::{http::header, web, App, HttpServer};
use cookbook_api::models::AppState;
use cookbook_api::routes::{
    health_check, get_categories, create_category, get_category, edit_category, delete_category,
    get_recipes, get_recipes_by_category, create_recipe, get_recipe, edit_recipe, delete_recipe
};
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "actix_web=info");
    }
    dotenv().ok();
    env_logger::init();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = match PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
    {
        Ok(pool) => {
            println!("✅Connection to the database is successful!");
            pool
        }
        Err(err) => {
            println!("🔥 Failed to connect to the database: {:?}", err);
            std::process::exit(1);
        }
    };

    println!("🚀 Server started successfully");

    HttpServer::new(move || {
        // let cors = Cors::default()
        //     .allowed_origin("http://localhost:3000")
        //     .allowed_methods(vec!["GET", "POST", "PATCH", "DELETE"])
        //     .allowed_headers(vec![
        //         header::CONTENT_TYPE,
        //         header::AUTHORIZATION,
        //         header::ACCEPT,
        //     ])
        //     .supports_credentials();
        App::new()
            .app_data(web::Data::new(AppState { db: pool.clone() }))
            // .configure(handler::config)
            // .wrap(cors)
            .wrap(Logger::default())
            .route("api/healthcheck", web::get().to(health_check))
            .route("api/categories", web::get().to(get_categories))
            .route("api/category", web::post().to(create_category))
            .route("api/category/{id}", web::get().to(get_category))
            .route("api/category/{id}", web::patch().to(edit_category))
            .route("api/category/{id}", web::delete().to(delete_category))
            .route("api/recipes", web::get().to(get_recipes))
            .route("api/recipes/category/{id}", web::get().to(get_recipes_by_category))
            .route("api/recipe", web::post().to(create_recipe))
            .route("api/recipe/{id}", web::get().to(get_recipe))
            .route("api/recipe/{id}", web::patch().to(edit_recipe))
            .route("api/recipe/{id}", web::delete().to(delete_recipe))
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}
