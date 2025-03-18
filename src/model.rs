use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, specta::Type, sqlx::Type)]
pub struct Product {
    pub id: i64,
    pub name: String,
    pub price: f64,
    pub created_at: i64,
}
