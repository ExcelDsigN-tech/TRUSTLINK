use sqlx::PgPool;
use tokio::time::{sleep, Duration};
use tracing::{error, info, warn};

use crate::config::Config;
use crate::db;

pub struct EventPoller {
    config: Config,
    pool: PgPool,
    contract_id: String,
}

impl EventPoller {
    pub fn new(config: Config, pool: PgPool) -> Self {
        let contract_id = config.contract_id.clone();
        Self { config, pool, contract_id }
    }

    pub async fn run(&self) {
        info!("Event poller starting for contract {}", self.contract_id);

        let mut current_ledger = db::get_cursor(&self.pool, &self.contract_id)
            .await
            .unwrap_or(self.config.start_ledger as i64);

        loop {
            match self.poll_ledger(current_ledger).await {
                Ok(Some(next_ledger)) => {
                    current_ledger = next_ledger;
                    db::upsert_cursor(&self.pool, &self.contract_id, current_ledger).await;
                }
                Ok(None) => {
                    sleep(Duration::from_millis(self.config.poll_interval_ms)).await;
                }
                Err(e) => {
                    error!("Poll error: {e}");
                    sleep(Duration::from_secs(5)).await;
                }
            }
        }
    }

    async fn poll_ledger(&self, ledger: i64) -> Result<Option<i64>, String> {
        let events = self.fetch_events(ledger).await.map_err(|e| e.to_string())?;
        if events.is_empty() {
            return Ok(None);
        }
        for event in &events {
            self.process_event(event).await.map_err(|e| e.to_string())?;
        }
        Ok(Some(ledger + 1))
    }

    async fn fetch_events(&self, ledger: i64) -> Result<Vec<SorobanEvent>, reqwest::Error> {
        let client = reqwest::Client::new();
        let body = serde_json::json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "getEvents",
            "params": {
                "startLedger": ledger,
                "filters": [{
                    "type": "contract",
                    "contractIds": [self.contract_id]
                }],
                "pagination": { "limit": 100 }
            }
        });

        let resp = client.post(&self.config.soroban_rpc_url).json(&body).send().await?;
        let result: serde_json::Value = resp.json().await?;

        let events_val = result.pointer("/result/events").and_then(|v| v.as_array());
        let mut parsed = Vec::new();

        if let Some(events) = events_val {
            for event_val in events {
                if let Some(event) = SorobanEvent::from_json(event_val) {
                    parsed.push(event);
                }
            }
        }
        Ok(parsed)
    }

    async fn process_event(&self, event: &SorobanEvent) -> Result<(), String> {
        match event.topic_name.as_str() {
            "EscrowCreated" => self.handle_escrow_created(event).await,
            "EscrowFunded" => self.handle_escrow_funded(event).await,
            "MilestoneApproved" => self.handle_milestone_approved(event).await,
            "MilestoneReleased" => self.handle_milestone_released(event).await,
            "DisputeRaised" => self.handle_dispute_raised(event).await,
            "DisputeResolved" => self.handle_dispute_resolved(event).await,
            "EscrowRefunded" => self.handle_escrow_refunded(event).await,
            _ => {
                warn!("Unknown event topic: {}", event.topic_name);
                Ok(())
            }
        }
    }

    async fn handle_escrow_created(&self, event: &SorobanEvent) -> Result<(), String> {
        let escrow_id = event.topic_args.first().copied().unwrap_or(0);
        info!("EscrowCreated: id={escrow_id}");
        db::insert_escrow(
            &self.pool, escrow_id, "", "", "", 0.0,
            "Active", u32::MAX as i64, 0, Some(&event.tx_hash),
        )
        .await;
        Ok(())
    }

    async fn handle_escrow_funded(&self, event: &SorobanEvent) -> Result<(), String> {
        let escrow_id = event.topic_args.first().copied().unwrap_or(0);
        info!("EscrowFunded: id={escrow_id}");
        db::update_escrow_status(&self.pool, escrow_id, "Funded").await;
        Ok(())
    }

    async fn handle_milestone_approved(&self, _event: &SorobanEvent) -> Result<(), String> {
        Ok(())
    }

    async fn handle_milestone_released(&self, event: &SorobanEvent) -> Result<(), String> {
        let escrow_id = event.topic_args.first().copied().unwrap_or(0);
        let amount: f64 = event.body_args.first().and_then(|s| s.parse().ok()).unwrap_or(0.0);

        info!("MilestoneReleased: escrow={escrow_id}, amount={amount}");

        let escrow = db::get_escrow_by_id(&self.pool, escrow_id).await;
        if let Some(e) = escrow {
            let new_released = e.released_amount + amount;
            let is_complete = new_released >= e.total_amount;
            let status = if is_complete { "Released" } else { "Funded" };
            db::update_escrow_released(&self.pool, escrow_id, new_released, status).await;

            if is_complete {
                db::insert_completion(
                    &self.pool, &e.client_address, escrow_id, &e.freelancer_address,
                    e.total_amount, false,
                )
                .await;
                db::insert_completion(
                    &self.pool, &e.freelancer_address, escrow_id, &e.client_address,
                    e.total_amount, false,
                )
                .await;
                db::upsert_reputation(&self.pool, &e.client_address, 1, e.total_amount).await;
                db::upsert_reputation(&self.pool, &e.freelancer_address, 1, e.total_amount).await;
            }
        }
        Ok(())
    }

    async fn handle_dispute_raised(&self, event: &SorobanEvent) -> Result<(), String> {
        let escrow_id = event.topic_args.first().copied().unwrap_or(0);
        let dispute_id = event.topic_args.get(1).copied().unwrap_or(0);
        info!("DisputeRaised: escrow={escrow_id}, dispute={dispute_id}");
        db::insert_dispute(&self.pool, dispute_id, escrow_id, "", "").await;
        db::update_escrow_status(&self.pool, escrow_id, "Disputed").await;
        Ok(())
    }

    async fn handle_dispute_resolved(&self, event: &SorobanEvent) -> Result<(), String> {
        let escrow_id = event.topic_args.first().copied().unwrap_or(0);
        let dispute_id = event.topic_args.get(1).copied().unwrap_or(0);
        info!("DisputeResolved: escrow={escrow_id}, dispute={dispute_id}");
        db::update_dispute_status(&self.pool, dispute_id, "Resolved", 0).await;
        Ok(())
    }

    async fn handle_escrow_refunded(&self, event: &SorobanEvent) -> Result<(), String> {
        let escrow_id = event.topic_args.first().copied().unwrap_or(0);
        info!("EscrowRefunded: id={escrow_id}");
        db::update_escrow_status(&self.pool, escrow_id, "Refunded").await;
        Ok(())
    }
}

struct SorobanEvent {
    tx_hash: String,
    topic_name: String,
    topic_args: Vec<i64>,
    body_args: Vec<String>,
}

impl SorobanEvent {
    fn from_json(val: &serde_json::Value) -> Option<Self> {
        let topic = val.pointer("/topic")?.as_array()?;
        let topic_name = topic.first()?.as_str()?.to_string();
        let topic_args: Vec<i64> = topic.iter().skip(1).filter_map(|t| t.as_i64()).collect();
        let body = val.get("value").or_else(|| val.get("body"));

        let body_args: Vec<String> = match body {
            Some(v) if v.is_array() => v
                .as_array()
                .map(|arr| arr.iter().filter_map(|x| x.as_str().map(String::from)).collect())
                .unwrap_or_default(),
            Some(v) => vec![v.to_string()],
            None => vec![],
        };

        let tx_hash = val.get("id").and_then(|v| v.as_str()).unwrap_or("").to_string();

        Some(Self { tx_hash, topic_name, topic_args, body_args })
    }
}
