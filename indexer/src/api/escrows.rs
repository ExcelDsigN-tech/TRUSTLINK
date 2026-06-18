use axum::{extract::{Path, Query, State}, Json, Router};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::sync::Arc;

use crate::db;

#[derive(Deserialize)]
pub struct EscrowQuery {
    status: Option<String>,
    limit: Option<i64>,
    offset: Option<i64>,
}

#[derive(Serialize)]
pub struct EscrowsResponse {
    pub success: bool,
    pub data: Vec<db::EscrowRow>,
}

#[derive(Serialize)]
pub struct EscrowDetailResponse {
    pub success: bool,
    pub data: Option<db::EscrowRow>,
    pub milestones: Vec<db::MilestoneRow>,
}

pub fn escrows_routes(state: Arc<PgPool>) -> Router {
    Router::new()
        .route("/", axum::routing::get(list_escrows))
        .route("/{id}", axum::routing::get(get_escrow))
        .route("/{id}/milestones", axum::routing::get(get_escrow_milestones))
        .with_state(state)
}

async fn list_escrows(
    State(pool): State<Arc<PgPool>>,
    Query(query): Query<EscrowQuery>,
) -> Json<EscrowsResponse> {
    let limit = query.limit.unwrap_or(20).min(100);
    let offset = query.offset.unwrap_or(0);
    let escrows = db::list_escrows(&pool, query.status.as_deref(), limit, offset).await;
    Json(EscrowsResponse { success: true, data: escrows })
}

async fn get_escrow(
    State(pool): State<Arc<PgPool>>,
    Path(id): Path<i64>,
) -> Json<EscrowDetailResponse> {
    let escrow = db::get_escrow_by_id(&pool, id).await;
    let milestones = if let Some(ref e) = escrow {
        db::get_milestones(&pool, e.id).await
    } else {
        vec![]
    };
    Json(EscrowDetailResponse { success: escrow.is_some(), data: escrow, milestones })
}

async fn get_escrow_milestones(
    State(pool): State<Arc<PgPool>>,
    Path(id): Path<i64>,
) -> Json<Vec<db::MilestoneRow>> {
    Json(db::get_milestones(&pool, id).await)
}
