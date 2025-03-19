mod db;
mod model;

use actix_files as fs;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use specta_typescript::{BigIntExportBehavior, Typescript};
use stripe::{
    CheckoutSession, CheckoutSessionMode, Client as StripeClient, CreateCheckoutSession,
    CreateCheckoutSessionLineItems, CreateCheckoutSessionLineItemsPriceData,
    CreateCheckoutSessionLineItemsPriceDataProductData, Currency,
};
use db::{get_all_products, init_db};
use serde::Deserialize;
use sqlx::SqlitePool;
use std::{env, path::PathBuf, sync::Arc};
use tracing::Level;

#[derive(Deserialize)]
struct CreateCheckoutSessionRequest {
    // Amount in cents (for example: $10.00 -> 1000)
    amount: i64,
}

async fn create_checkout_session(
    body: web::Json<CreateCheckoutSessionRequest>,
    stripe_client: web::Data<StripeClient>,
) -> impl Responder {
    // Build a new Checkout Session creation request.
    let mut params = CreateCheckoutSession::new();
    params.success_url = Some("http://localhost:5526/checkout-success".into());
    params.cancel_url = Some("http://localhost:5526/checkout-cancel".into());
    params.mode = Some(CheckoutSessionMode::Payment);
    params.line_items = Some(vec![CreateCheckoutSessionLineItems {
        quantity: Some(1),
        price_data: Some(CreateCheckoutSessionLineItemsPriceData {
            currency: Currency::GBP,
            unit_amount: Some(body.amount), // amount in cents
            product_data: Some(CreateCheckoutSessionLineItemsPriceDataProductData {
                name: "Order from The Good Shop".into(),
                ..Default::default()
            }),
            ..Default::default()
        }),
        ..Default::default()
    }]);

    match CheckoutSession::create(&stripe_client, params).await {
        Ok(session) => {
            if let Some(url) = session.url {
                HttpResponse::Ok().json(serde_json::json!({ "url": url }))
            } else {
                HttpResponse::InternalServerError()
                    .json(serde_json::json!({ "error": "No URL returned" }))
            }
        }
        Err(err) => {
            eprintln!("Stripe error: {:?}", err);
            HttpResponse::InternalServerError()
                .json(serde_json::json!({ "error": format!("{:?}", err) }))
        }
    }
}

async fn products_endpoint(pool: web::Data<Arc<SqlitePool>>) -> impl Responder {
    let products = get_all_products(&pool).await;
    HttpResponse::Ok().json(products)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();

    // Load environment variables from .env
    dotenv::dotenv().ok();

    // Database setup
    let database_url = env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite://mydb.sqlite".into());
    let pool = init_db(&database_url).await;
    let shared_pool = Arc::new(pool);

    // Stripe client setup: load your Stripe secret key from the environment.
    let stripe_secret = env::var("STRIPE_API_KEY").expect("STRIPE_API_KEY env var not found");
    let stripe_client = StripeClient::new(stripe_secret);

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

    // Build and run the HTTP server.
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(shared_pool.clone()))
            .app_data(web::Data::new(stripe_client.clone()))
            // API endpoints:
            .route("/api/products", web::get().to(products_endpoint))
            .route(
                "/api/create-checkout-session",
                web::post().to(create_checkout_session),
            )
            // Serve static files for the frontend from the Vike build (e.g., "dist/client").
            // This serves any file in dist/client, and falls back to index.html.
            .service(
                fs::Files::new("/", "./thegoodshop/dist/client")
                    .index_file("index.html")
                    .use_last_modified(true),
            )
    })
    .bind(("127.0.0.1", 5526))?
    .run()
    .await
}
