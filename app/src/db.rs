// DB

// This file provide the pool connection for the database access

use std::sync::Arc;

use anyhow::Result;
use sqlx::{
    postgres::{PgPool, PgPoolOptions},
    Pool, Postgres,
};

// Type
type AppPool = Arc<Pool<Postgres>>;

#[derive(Debug, Clone)]
pub struct AppState {
    pub db: AppPool,
}

pub async fn conn() -> Result<PgPool> {
    let db_url = std::env::var("DATABASE_URL").expect("Unable to find db url");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await?;

    Ok(pool)
}
