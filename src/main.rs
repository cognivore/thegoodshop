mod db;
mod model;

use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use db::{get_all_products, init_db};
use specta_typescript::{BigIntExportBehavior, Typescript};
use sqlx::SqlitePool;
use tracing::Level;
use std::{path::PathBuf, sync::Arc};

async fn products_endpoint(pool: web::Data<Arc<SqlitePool>>) -> impl Responder {
    let products = get_all_products(&pool).await;
    HttpResponse::Ok().json(products)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Set up tracing, which shows info into console
    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .init();

    let database_url =
        std::env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite://mydb.sqlite".into());
    let pool = init_db(&database_url).await;
    let shared_pool = Arc::new(pool);

    let mut frontend_dir = std::env::current_dir()?;

    fn find_package_json_dir(start_path: &std::path::Path) -> Option<PathBuf> {
        if start_path.join("package.json").exists() {
            return Some(start_path.to_path_buf());
        }

        std::fs::read_dir(start_path).ok().and_then(|entries| {
            entries
                .filter_map(Result::ok)
                .map(|entry| entry.path())
                .filter(|path| path.is_dir())
                .find_map(|dir| find_package_json_dir(&dir))
        })
    }
    frontend_dir =
        find_package_json_dir(&frontend_dir).expect("Could not find package.json in any directory");

    let types_path = frontend_dir.join("src").join("types").join("generated.ts");

    if let Some(parent) = types_path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    tracing::info!("Exporting types");
    specta_util::export()
        .export_to(
            Typescript::new().bigint(BigIntExportBehavior::String),
            &types_path,
        )
        .expect("Failed to export types");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(shared_pool.clone()))
            .route("/api/products", web::get().to(products_endpoint))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
