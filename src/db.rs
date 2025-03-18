use sqlx::{Pool, Sqlite, sqlite::SqlitePoolOptions};
use crate::model::Product;

pub async fn init_db(database_url: &str) -> Pool<Sqlite> {
    let pool = SqlitePoolOptions::new()
        .connect(database_url)
        .await
        .expect("Failed to connect to the database");

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to run migrations");

    pool
}

pub async fn get_all_products(pool: &Pool<Sqlite>) -> Vec<Product> {
    let rows = sqlx::query_as!(
        Product,
        r#"SELECT id, name, price, created_at as "created_at: i64" FROM products"#
    )
    .fetch_all(pool)
    .await
    .expect("Failed to fetch products");

    rows
}
