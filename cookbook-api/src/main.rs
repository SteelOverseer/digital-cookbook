use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::{http::header, web, App, HttpServer};
use cookbook_api::models::AppState;
use cookbook_api::routes::{
    add_tag, 
    create_category,
    create_ingredient,
    create_instruction,
    create_recipe,
    create_tag,
    delete_category,
    delete_instruction,
    delete_recipe,
    delete_tag,
    edit_category,
    edit_instruction,
    edit_recipe,
    edit_tag,
    get_categories,
    get_category,
    get_instructions_for_recipe,
    get_recipe,
    get_recipes,
    get_recipes_by_category,
    get_tags,
    health_check,
    remove_tag,
    get_ingredients_for_recipe,
    edit_ingredient,
    delete_ingredient
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
        let cors = Cors::default()
            .allowed_origin("http://localhost:5173")
            .allowed_methods(vec!["GET", "POST", "PATCH", "DELETE"])
            .allowed_headers(vec![
                header::CONTENT_TYPE,
                header::AUTHORIZATION,
                header::ACCEPT,
            ])
            .supports_credentials();
        App::new()
            .app_data(web::Data::new(AppState { db: pool.clone() }))
            // .configure(handler::config)
            .wrap(cors)
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
            .route("api/instructions/recipe/{id}", web::get().to(get_instructions_for_recipe))
            .route("api/instruction", web::post().to(create_instruction))
            .route("api/instruction/{id}", web::patch().to(edit_instruction))
            .route("api/instruction/{id}", web::delete().to(delete_instruction))
            .route("api/tags", web::get().to(get_tags))
            .route("api/tag", web::post().to(create_tag))
            .route("api/tag/{id}", web::patch().to(edit_tag))
            .route("api/tag/{id}", web::delete().to(delete_tag))
            .route("api/recipe/{recipe_id}/tag/{tag_id}", web::post().to(add_tag))
            .route("api/recipe/{recipe_id}/tag/{tag_id}", web::delete().to(remove_tag))
            .route("api/ingredient", web::post().to(create_ingredient))
            .route("api/ingredient/{recipe_id}", web::get().to(get_ingredients_for_recipe))
            .route("api/ingredient/{id}", web::patch().to(edit_ingredient))
            .route("api/ingredient/{id}", web::delete().to(delete_ingredient))
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}
