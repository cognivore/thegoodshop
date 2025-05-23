mod db;
mod model;

use actix_files as fs;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use db::{get_all_products, init_db};
use serde::Deserialize;
use specta_typescript::{BigIntExportBehavior, Typescript};
use sqlx::SqlitePool;
use std::{env, fs as std_fs, path::PathBuf, sync::Arc};
use stripe::{
    CheckoutSession, CheckoutSessionMode, Client as StripeClient, CreateCheckoutSession,
    CreateCheckoutSessionLineItems, CreateCheckoutSessionLineItemsPriceData,
    CreateCheckoutSessionLineItemsPriceDataProductData, Currency,
};
use tracing::{debug, error, info, Level};

#[allow(dead_code)]
#[derive(Deserialize)]
struct CheckoutProduct {
    id: u64,
    name: String,
    price: f64,
    quantity: u64,
}

#[derive(Deserialize)]
struct CreateCheckoutSessionRequest {
    products: Vec<CheckoutProduct>,
}

async fn create_checkout_session(
    body: web::Json<CreateCheckoutSessionRequest>,
    stripe_client: web::Data<StripeClient>,
) -> impl Responder {
    let mut params = CreateCheckoutSession::new();
    params.success_url = Some("http://localhost:5526/checkout-success".into());
    params.cancel_url = Some("http://localhost:5526/checkout-cancel".into());
    params.mode = Some(CheckoutSessionMode::Payment);

    params.line_items = Some(
        body.products
            .iter()
            .map(|product| CreateCheckoutSessionLineItems {
                quantity: Some(product.quantity),
                price_data: Some(CreateCheckoutSessionLineItemsPriceData {
                    currency: Currency::GBP,
                    unit_amount: Some((product.price * 100.0) as i64), // convert to pence
                    product_data: Some(CreateCheckoutSessionLineItemsPriceDataProductData {
                        name: product.name.clone(),
                        ..Default::default()
                    }),
                    ..Default::default()
                }),
                ..Default::default()
            })
            .collect(),
    );

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
    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .init();

    info!("Starting The Good Shop service");

    // Load environment variables from .env
    match dotenv::dotenv() {
        Ok(_) => debug!("Loaded environment from .env file"),
        Err(e) => debug!("Could not load .env file: {}", e),
    }

    // Database setup
    let database_url = env::var("DATABASE_URL").unwrap_or_else(|_| {
        debug!("DATABASE_URL not set, using default");
        "sqlite://./db/shop.db".into()
    });

    debug!("Using database URL: {}", database_url);

    // Check permissions on migrations directory
    let migrations_path = PathBuf::from("./migrations");
    if migrations_path.exists() {
        debug!("Migrations directory exists at {:?}", migrations_path);
        match std_fs::read_dir(&migrations_path) {
            Ok(entries) => {
                debug!("Migrations directory is readable");
                for entry in entries {
                    match entry {
                        Ok(entry) => debug!("Migration file: {:?}", entry.path()),
                        Err(e) => error!("Error reading migration entry: {}", e),
                    }
                }
            }
            Err(e) => error!("Cannot read migrations directory: {}", e),
        }
    } else {
        error!(
            "Migrations directory does not exist at {:?}",
            migrations_path
        );
    }

    // Check database file and directory
    if let Some(path) = database_url.strip_prefix("sqlite://") {
        let db_path = PathBuf::from(path);
        if let Some(parent) = db_path.parent() {
            if parent.exists() {
                debug!("Database directory exists at {:?}", parent);
                // Check if we can write to this directory
                match std_fs::File::create(parent.join("test_write_permission.tmp")) {
                    Ok(_) => {
                        debug!("Database directory is writable");
                        let _ = std_fs::remove_file(parent.join("test_write_permission.tmp"));
                    }
                    Err(e) => error!("Cannot write to database directory: {}", e),
                }
            } else {
                error!("Database directory does not exist at {:?}", parent);
            }
        }

        if db_path.exists() {
            debug!("Database file exists at {:?}", db_path);
            match std_fs::File::options().write(true).open(&db_path) {
                Ok(_) => debug!("Database file is writable"),
                Err(e) => error!("Cannot write to database file: {}", e),
            }
        } else {
            error!("Database file does not exist at {:?}", db_path);
        }
    }

    info!("Initializing database connection pool");
    let pool = init_db(&database_url).await;
    let shared_pool = Arc::new(pool);

    // Stripe client setup: load your Stripe secret key from the environment.
    debug!("Setting up Stripe client");
    let stripe_secret = match env::var("STRIPE_API_KEY") {
        Ok(secret) => {
            debug!("STRIPE_API_KEY found");
            secret
        }
        Err(e) => {
            error!("STRIPE_API_KEY not found: {}", e);
            return Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "STRIPE_API_KEY not found",
            ));
        }
    };

    let stripe_client = StripeClient::new(stripe_secret);
    debug!("Stripe client initialized");

    // Skip Specta type export in production or if directories are read-only
    let dev_mode = env::var("DEV_MODE")
        .map(|v| v == "1" || v.to_lowercase() == "true")
        .unwrap_or(false);

    if dev_mode {
        debug!("Development mode detected, exporting types");
        debug!("Looking for frontend directory containing package.json");
        let mut frontend_dir = std::env::current_dir()?;
        debug!("Current directory: {:?}", frontend_dir);

        fn find_package_json_dir(start_path: &std::path::Path) -> Option<PathBuf> {
            debug!("Checking for package.json in: {:?}", start_path);
            let package_json_path = start_path.join("package.json");
            debug!("Looking for: {:?}", package_json_path);
            if package_json_path.exists() {
                debug!("Found package.json at: {:?}", package_json_path);
                return Some(start_path.to_path_buf());
            }

            match std_fs::read_dir(start_path) {
                Ok(entries) => {
                    debug!("Reading directory: {:?}", start_path);
                    for entry_result in entries {
                        match entry_result {
                            Ok(entry) => {
                                let path = entry.path();
                                if path.is_dir() {
                                    debug!("Checking subdirectory: {:?}", path);
                                    if let Some(found_dir) = find_package_json_dir(&path) {
                                        return Some(found_dir);
                                    }
                                }
                            }
                            Err(e) => error!("Error reading directory entry: {}", e),
                        }
                    }
                    None
                }
                Err(e) => {
                    error!("Cannot read directory {:?}: {}", start_path, e);
                    None
                }
            }
        }

        frontend_dir = match find_package_json_dir(&frontend_dir) {
            Some(dir) => {
                debug!("Found frontend directory: {:?}", dir);
                dir
            }
            None => {
                error!("Could not find package.json in any directory");
                return Err(std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    "Could not find package.json in any directory",
                ));
            }
        };

        let types_path = frontend_dir.join("src").join("types").join("generated.ts");
        debug!("Types path: {:?}", types_path);

        if let Some(parent) = types_path.parent() {
            debug!("Creating types directory: {:?}", parent);
            match std_fs::create_dir_all(parent) {
                Ok(_) => debug!("Created types directory"),
                Err(e) => {
                    error!("Failed to create types directory: {}", e);
                    // Don't return, just log the error and continue without type generation
                }
            }
        }

        debug!("Exporting types");
        match specta_util::export().export_to(
            Typescript::new().bigint(BigIntExportBehavior::String),
            &types_path,
        ) {
            Ok(_) => debug!("Successfully exported types"),
            Err(e) => {
                error!("Failed to export types: {}", e);
                // Don't return, just log the error and continue without type generation
            }
        };
    } else {
        info!("Production mode detected, skipping type export");
    }

    // Build and run the HTTP server.
    info!("Starting HTTP server on http://127.0.0.1:5526");

    // Check if we can access the frontend static files directory
    let static_dir = PathBuf::from("./thegoodshop/dist/client");
    if static_dir.exists() {
        debug!("Static files directory exists: {:?}", static_dir);
        match std_fs::read_dir(&static_dir) {
            Ok(_) => debug!("Static files directory is readable"),
            Err(e) => error!("Cannot read static files directory: {}", e),
        }
    } else {
        error!("Static files directory does not exist: {:?}", static_dir);
    }

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
