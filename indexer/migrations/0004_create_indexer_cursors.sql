CREATE TABLE IF NOT EXISTS indexer_cursors (
    contract_id TEXT PRIMARY KEY,
    last_ledger BIGINT NOT NULL DEFAULT 0,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
