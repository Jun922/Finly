mod models;      // src/models/mod.rs を読み込む
mod controllers; // src/controllers/mod.rs を読み込む

use axum::Router;
use axum::http::Method;
use sqlx::sqlite::SqlitePool;
use tower_http::cors::{Any, CorsLayer};

#[tokio::main]
async fn main() {
    let pool = SqlitePool::connect("sqlite:finly.db")
        .await
        .expect("Failed to connect to DB");

    // CORSの設定
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_headers(Any);

    // APIルートの構築
    // controllers::accounting::accounting_routes() を呼び出す
    let api_routes = Router::new()
        .nest("/accountings", controllers::accounting::accounting_routes());

    let app = Router::new()
        .nest("/api", api_routes)
        .layer(cors)
        .with_state(pool);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080").await.unwrap();
    println!("Server running on http://127.0.0.1:8080");
    axum::serve(listener, app).await.unwrap();
}
