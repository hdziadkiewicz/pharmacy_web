mod db;
mod models {
    pub mod medicine;
}
mod routes {
    pub mod medicines;
}

use axum::Router;
use dotenvy::dotenv;
use routes::medicines::{AppState, router};
use sqlx::sqlite::SqlitePoolOptions;
use std::net::SocketAddr;
use tracing_subscriber;

#[tokio::main]
async fn main() {
    dotenv().ok(); // Load .env
    tracing_subscriber::fmt::init(); // Logging

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL not set");
    let pool = SqlitePoolOptions::new()
        .connect(&database_url)
        .await
        .expect("Database connection failed");

    let app_state = AppState { db: pool.clone() };

    let app = router().with_state(app_state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("🚀 Server running at http://{}", addr);

    use axum::serve;
    use tokio::net::TcpListener;

    let listener = TcpListener::bind(addr).await.unwrap();
    serve(listener, app).await.unwrap();
}
