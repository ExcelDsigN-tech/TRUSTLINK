use axum::{extract::{Path, State}, Json, Router};
use serde::Serialize;
use sqlx::PgPool;
use std::sync::Arc;

use crate::db;

#[derive(Serialize)]
pub struct DisputeDetailResponse {
    pub success: bool,
    pub data: Option<db::DisputeRow>,
}

pub fn disputes_routes(state: Arc<PgPool>) -> Router {
    Router::new()
        .route("/{id}", axum::routing::get(get_dispute))
        .with_state(state)
}

async fn get_dispute(
    State(pool): State<Arc<PgPool>>,
    Path(id): Path<i64>,
) -> Json<DisputeDetailResponse> {
    let dispute = sqlx::query_as::<_, db::DisputeRow>(
        "SELECT * FROM disputes WHERE contract_dispute_id = $1",
    )
    .bind(id)
    .fetch_optional(&*pool)
    .await
    .ok()
    .flatten();

    Json(DisputeDetailResponse { success: dispute.is_some(), data: dispute })
}
