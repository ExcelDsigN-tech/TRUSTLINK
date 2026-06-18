use axum::{extract::{Path, Query, State}, Json, Router};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::sync::Arc;

use crate::db;

#[derive(Deserialize)]
pub struct HistoryQuery {
    limit: Option<i64>,
    offset: Option<i64>,
}

#[derive(Serialize)]
pub struct ReputationResponse {
    pub success: bool,
    pub data: Option<db::ReputationRow>,
}

#[derive(Serialize)]
pub struct HistoryResponse {
    pub success: bool,
    pub data: Vec<db::CompletionRow>,
}

pub fn reputation_routes(state: Arc<PgPool>) -> Router {
    Router::new()
        .route("/{address}", axum::routing::get(get_reputation))
        .route("/{address}/history", axum::routing::get(get_history))
        .with_state(state)
}

async fn get_reputation(
    State(pool): State<Arc<PgPool>>,
    Path(address): Path<String>,
) -> Json<ReputationResponse> {
    let rep = db::get_reputation(&pool, &address).await;
    Json(ReputationResponse { success: rep.is_some(), data: rep })
}

async fn get_history(
    State(pool): State<Arc<PgPool>>,
    Path(address): Path<String>,
    Query(query): Query<HistoryQuery>,
) -> Json<HistoryResponse> {
    let limit = query.limit.unwrap_or(20).min(100);
    let offset = query.offset.unwrap_or(0);
    let data = db::get_completion_history(&pool, &address, limit, offset).await;
    Json(HistoryResponse { success: true, data })
}
