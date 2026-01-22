use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use ts_rs::TS;

// Entity: DBのテーブル構造そのもの
#[derive(Serialize, Deserialize, TS, sqlx::FromRow)]
#[ts(export, export_to = "../../frontend/src/types/accountings.ts")]
pub struct Accounting {
    pub accounting_id: i32,
    pub name: String,
    pub income_expenditure: i32,
    pub price: i32,
    pub registered_at: DateTime<Utc>,
    pub memo: Option<String>,
    pub created_by: String,
    pub created_at: DateTime<Utc>,
    pub updated_by: String,
    pub updated_at: DateTime<Utc>,
}

// CreateDTO: 登録時の入力データ
#[derive(Deserialize)]
pub struct CreateAccounting {
    pub name: String,
    pub income_expenditure: i32,
    pub price: i32,
    pub memo: Option<String>,
}

// UpdateDTO: 更新時の入力データ
#[derive(Deserialize)]
pub struct UpdateAccounting {
    pub name: String,
    pub income_expenditure: i32,
    pub price: i32,
    pub memo: Option<String>,
    pub updated_by: String,
}
