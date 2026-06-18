mod escrows;
mod disputes;
mod reputation;

pub use escrows::{EscrowsResponse, EscrowDetailResponse};
pub use disputes::DisputeDetailResponse;
pub use reputation::{ReputationResponse, HistoryResponse};

use axum::Router;
use sqlx::PgPool;
use std::sync::Arc;

use crate::ws;

pub fn build_routes(pool: PgPool) -> Router {
    let state = Arc::new(pool);

    Router::new()
        .merge(escrows::escrows_routes(state.clone()))
        .merge(disputes::disputes_routes(state.clone()))
        .merge(reputation::reputation_routes(state.clone()))
        .merge(ws::ws_routes(state))
}
