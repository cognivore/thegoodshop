use crate::model::Product;
use sqlx::{sqlite::SqlitePoolOptions, Pool, Sqlite};
use tracing::{debug, error, info};

pub async fn init_db(database_url: &str) -> Pool<Sqlite> {
    debug!("Initializing database with URL: {}", database_url);

    let pool = match SqlitePoolOptions::new().connect(database_url).await {
        Ok(pool) => {
            debug!("Successfully connected to database");
            pool
        }
        Err(e) => {
            error!("Failed to connect to database: {}", e);
            panic!("Failed to connect to the database: {}", e);
        }
    };

    debug!("Running migrations from ./migrations directory");
    match sqlx::migrate!("./migrations").run(&pool).await {
        Ok(_) => {
            debug!("Successfully ran migrations");
        }
        Err(e) => {
            error!("Failed to run migrations: {}", e);
            panic!("Failed to run migrations: {}", e);
        }
    };

    info!("Database initialization complete");
    pool
}

pub async fn get_all_products(pool: &Pool<Sqlite>) -> Vec<Product> {
    debug!("Fetching all products from database");

    let rows = match sqlx::query_as!(
        Product,
        r#"SELECT id, name, price, created_at as "created_at: i64" FROM products"#
    )
    .fetch_all(pool)
    .await
    {
        Ok(rows) => {
            debug!("Successfully fetched {} products", rows.len());
            rows
        }
        Err(e) => {
            error!("Failed to fetch products: {}", e);
            panic!("Failed to fetch products: {}", e);
        }
    };

    rows
}
