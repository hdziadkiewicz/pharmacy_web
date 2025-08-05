use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Medicine {
    pub id: i64,
    pub name: String,
    pub quantity: i32,
}
