use chrono::{DateTime, Utc};
use sqlx::postgres::PgPoolOptions;
use sqlx::{FromRow, PgPool};

#[derive(Clone, Debug, FromRow, serde::Serialize)]
pub struct EscrowRow {
    pub id: i64,
    pub contract_escrow_id: i64,
    pub client_address: String,
    pub freelancer_address: String,
    pub token_address: String,
    pub total_amount: f64,
    pub released_amount: f64,
    pub status: String,
    pub expiry_ledger: i64,
    pub milestone_count: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub tx_hash: Option<String>,
}

#[derive(Clone, Debug, FromRow, serde::Serialize)]
pub struct MilestoneRow {
    pub id: i64,
    pub escrow_id: i64,
    pub milestone_index: i32,
    pub percentage: i32,
    pub description: String,
    pub is_approved: bool,
    pub is_released: bool,
}

#[derive(Clone, Debug, FromRow, serde::Serialize)]
pub struct DisputeRow {
    pub id: i64,
    pub contract_dispute_id: i64,
    pub escrow_id: i64,
    pub raised_by: String,
    pub reason: String,
    pub status: String,
    pub evidence_hashes: Vec<String>,
    pub verdict_ledger: i64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Clone, Debug, FromRow, serde::Serialize)]
pub struct ReputationRow {
    pub address: String,
    pub total_deals: i32,
    pub completed_deals: i32,
    pub disputed_deals: i32,
    pub total_volume: f64,
}

#[derive(Clone, Debug, FromRow, serde::Serialize)]
pub struct CompletionRow {
    pub id: i64,
    pub address: String,
    pub escrow_id: i64,
    pub counterparty: String,
    pub amount: f64,
    pub completed_at: DateTime<Utc>,
    pub had_dispute: bool,
}

#[derive(Clone, Debug, FromRow, serde::Serialize)]
pub struct CursorRow {
    pub contract_id: String,
    pub last_ledger: i64,
}

pub async fn create_pool(database_url: &str) -> PgPool {
    PgPoolOptions::new()
        .max_connections(10)
        .connect(database_url)
        .await
        .expect("Failed to create database pool")
}

pub async fn run_migrations(pool: &PgPool) {
    sqlx::migrate!("./migrations")
        .run(pool)
        .await
        .expect("Failed to run migrations");
}

pub async fn get_cursor(pool: &PgPool, contract_id: &str) -> Option<i64> {
    sqlx::query_scalar::<_, i64>(
        "SELECT last_ledger FROM indexer_cursors WHERE contract_id = $1",
    )
    .bind(contract_id)
    .fetch_optional(pool)
    .await
    .ok()
    .flatten()
}

pub async fn upsert_cursor(pool: &PgPool, contract_id: &str, ledger: i64) {
    sqlx::query(
        "INSERT INTO indexer_cursors (contract_id, last_ledger, updated_at)
         VALUES ($1, $2, NOW())
         ON CONFLICT (contract_id)
         DO UPDATE SET last_ledger = $2, updated_at = NOW()",
    )
    .bind(contract_id)
    .bind(ledger)
    .execute(pool)
    .await
    .ok();
}

pub async fn insert_escrow(
    pool: &PgPool,
    eid: i64,
    client: &str,
    freelancer: &str,
    token: &str,
    total_amount: f64,
    status: &str,
    expiry: i64,
    milestone_count: i32,
    tx_hash: Option<&str>,
) {
    sqlx::query(
        "INSERT INTO escrows (contract_escrow_id, client_address, freelancer_address,
         token_address, total_amount, released_amount, status, expiry_ledger, milestone_count, tx_hash)
         VALUES ($1, $2, $3, $4, $5, 0, $6, $7, $8, $9)
         ON CONFLICT (contract_escrow_id)
         DO UPDATE SET status = $6, updated_at = NOW()",
    )
    .bind(eid)
    .bind(client)
    .bind(freelancer)
    .bind(token)
    .bind(total_amount)
    .bind(status)
    .bind(expiry)
    .bind(milestone_count)
    .bind(tx_hash)
    .execute(pool)
    .await
    .ok();
}

pub async fn update_escrow_status(pool: &PgPool, eid: i64, status: &str) {
    sqlx::query(
        "UPDATE escrows SET status = $1, updated_at = NOW() WHERE contract_escrow_id = $2",
    )
    .bind(status)
    .bind(eid)
    .execute(pool)
    .await
    .ok();
}

pub async fn update_escrow_released(pool: &PgPool, eid: i64, released: f64, status: &str) {
    sqlx::query(
        "UPDATE escrows SET released_amount = $1, status = $2, updated_at = NOW()
         WHERE contract_escrow_id = $3",
    )
    .bind(released)
    .bind(status)
    .bind(eid)
    .execute(pool)
    .await
    .ok();
}

pub async fn list_escrows(
    pool: &PgPool,
    status_filter: Option<&str>,
    limit: i64,
    offset: i64,
) -> Vec<EscrowRow> {
    if let Some(s) = status_filter {
        sqlx::query_as::<_, EscrowRow>(
            "SELECT * FROM escrows WHERE status = $1 ORDER BY created_at DESC LIMIT $2 OFFSET $3",
        )
        .bind(s)
        .bind(limit)
        .bind(offset)
        .fetch_all(pool)
        .await
        .unwrap_or_default()
    } else {
        sqlx::query_as::<_, EscrowRow>(
            "SELECT * FROM escrows ORDER BY created_at DESC LIMIT $1 OFFSET $2",
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(pool)
        .await
        .unwrap_or_default()
    }
}

pub async fn get_escrow_by_id(pool: &PgPool, eid: i64) -> Option<EscrowRow> {
    sqlx::query_as::<_, EscrowRow>(
        "SELECT * FROM escrows WHERE contract_escrow_id = $1",
    )
    .bind(eid)
    .fetch_optional(pool)
    .await
    .ok()
    .flatten()
}

pub async fn get_milestones(pool: &PgPool, escrow_id: i64) -> Vec<MilestoneRow> {
    sqlx::query_as::<_, MilestoneRow>(
        "SELECT * FROM milestones WHERE escrow_id = $1 ORDER BY milestone_index",
    )
    .bind(escrow_id)
    .fetch_all(pool)
    .await
    .unwrap_or_default()
}

pub async fn insert_dispute(
    pool: &PgPool,
    did: i64,
    escrow_id: i64,
    raised_by: &str,
    reason: &str,
) {
    sqlx::query(
        "INSERT INTO disputes (contract_dispute_id, escrow_id, raised_by, reason, status)
         VALUES ($1, $2, $3, $4, 'Open')
         ON CONFLICT (contract_dispute_id) DO NOTHING",
    )
    .bind(did)
    .bind(escrow_id)
    .bind(raised_by)
    .bind(reason)
    .execute(pool)
    .await
    .ok();
}

pub async fn update_dispute_status(pool: &PgPool, did: i64, status: &str, verdict_ledger: i64) {
    sqlx::query(
        "UPDATE disputes SET status = $1, verdict_ledger = $2, updated_at = NOW()
         WHERE contract_dispute_id = $3",
    )
    .bind(status)
    .bind(verdict_ledger)
    .bind(did)
    .execute(pool)
    .await
    .ok();
}

pub async fn insert_completion(
    pool: &PgPool,
    address: &str,
    escrow_id: i64,
    counterparty: &str,
    amount: f64,
    had_dispute: bool,
) {
    sqlx::query(
        "INSERT INTO completions (address, escrow_id, counterparty, amount, had_dispute)
         VALUES ($1, $2, $3, $4, $5)",
    )
    .bind(address)
    .bind(escrow_id)
    .bind(counterparty)
    .bind(amount)
    .bind(had_dispute)
    .execute(pool)
    .await
    .ok();
}

pub async fn upsert_reputation(
    pool: &PgPool,
    address: &str,
    delta_deals: i32,
    delta_volume: f64,
) {
    sqlx::query(
        "INSERT INTO reputations (address, total_deals, completed_deals, total_volume)
         VALUES ($1, 1, $2, $3)
         ON CONFLICT (address)
         DO UPDATE SET
           total_deals = reputations.total_deals + 1,
           completed_deals = reputations.completed_deals + $2,
           total_volume = reputations.total_volume + $3",
    )
    .bind(address)
    .bind(delta_deals)
    .bind(delta_volume)
    .execute(pool)
    .await
    .ok();
}

pub async fn get_reputation(pool: &PgPool, address: &str) -> Option<ReputationRow> {
    sqlx::query_as::<_, ReputationRow>(
        "SELECT * FROM reputations WHERE address = $1",
    )
    .bind(address)
    .fetch_optional(pool)
    .await
    .ok()
    .flatten()
}

pub async fn get_completion_history(
    pool: &PgPool,
    address: &str,
    limit: i64,
    offset: i64,
) -> Vec<CompletionRow> {
    sqlx::query_as::<_, CompletionRow>(
        "SELECT * FROM completions WHERE address = $1
         ORDER BY completed_at DESC LIMIT $2 OFFSET $3",
    )
    .bind(address)
    .bind(limit)
    .bind(offset)
    .fetch_all(pool)
    .await
    .unwrap_or_default()
}
