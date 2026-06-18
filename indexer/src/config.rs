use std::env;

#[derive(Clone, Debug)]
pub struct Config {
    pub database_url: String,
    pub soroban_rpc_url: String,
    pub contract_id: String,
    pub poll_interval_ms: u64,
    pub start_ledger: u32,
    pub host: String,
    pub port: u16,
}

impl Config {
    pub fn from_env() -> Self {
        Self {
            database_url: env::var("DATABASE_URL")
                .expect("DATABASE_URL must be set"),
            soroban_rpc_url: env::var("SOROBAN_RPC_URL")
                .unwrap_or_else(|_| "https://rpc-futurenet.stellar.org".into()),
            contract_id: env::var("CONTRACT_ID")
                .expect("CONTRACT_ID must be set"),
            poll_interval_ms: env::var("POLL_INTERVAL_MS")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(5000),
            start_ledger: env::var("START_LEDGER")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(1),
            host: env::var("HOST").unwrap_or_else(|_| "0.0.0.0".into()),
            port: env::var("PORT")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(3001),
        }
    }
}
