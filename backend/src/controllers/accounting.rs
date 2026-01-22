use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post, put, delete},
    Json, Router,
};
use sqlx::sqlite::SqlitePool;
use crate::models::accounting::{Accounting, CreateAccounting, UpdateAccounting};

// Accountingに関連するルートをまとめて定義
pub fn accounting_routes() -> Router<SqlitePool> {
    Router::new()
        .route("/", get(get_accountings))
        .route("/", post(create_accounting))
        .route("/:id", put(update_accounting))
        .route("/:id", delete(delete_accounting))
}

async fn get_accountings(State(pool): State<SqlitePool>) -> impl IntoResponse {
    let result = sqlx::query_as::<_, Accounting>(
        "SELECT * FROM accountings ORDER BY accounting_id ASC"
    )
    .fetch_all(&pool).await;

    match result {
        Ok(list) => Json(list).into_response(),
        Err(e) => {
            eprintln!("DB Error: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

async fn create_accounting(
    State(pool): State<SqlitePool>,
    Json(payload): Json<CreateAccounting>,
) -> impl IntoResponse {
    let result = sqlx::query(
        "INSERT INTO accountings (name, income_expenditure, price, memo, created_by, updated_by) VALUES (?, ?, ?, ?, 'admin', 'admin')"
    )
    .bind(&payload.name).bind(payload.income_expenditure).bind(payload.price).bind(&payload.memo)
    .execute(&pool).await;

    match result {
        Ok(_) => StatusCode::CREATED.into_response(),
        Err(e) => {
            eprintln!("Insert Error: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

async fn update_accounting(
    State(pool): State<SqlitePool>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateAccounting>,
) -> impl IntoResponse {
    let result = sqlx::query(
        "UPDATE accountings SET name = ?, income_expenditure = ?, price = ?, memo = ?, updated_by = ?, updated_at = CURRENT_TIMESTAMP WHERE accounting_id = ?"
    )
    .bind(&payload.name).bind(payload.income_expenditure).bind(payload.price).bind(&payload.memo).bind(&payload.updated_by).bind(id)
    .execute(&pool).await;

    match result {
        Ok(res) if res.rows_affected() > 0 => StatusCode::OK.into_response(),
        _ => StatusCode::NOT_FOUND.into_response(),
    }
}

async fn delete_accounting(State(pool): State<SqlitePool>, Path(id): Path<i32>) -> impl IntoResponse {
    let result = sqlx::query("DELETE FROM accountings WHERE accounting_id = ?")
        .bind(id).execute(&pool).await;

    match result {
        Ok(res) if res.rows_affected() > 0 => StatusCode::NO_CONTENT.into_response(),
        _ => StatusCode::NOT_FOUND.into_response(),
    }
}
