CREATE TABLE IF NOT EXISTS reputations (
    address TEXT PRIMARY KEY,
    total_deals INT NOT NULL DEFAULT 0,
    completed_deals INT NOT NULL DEFAULT 0,
    disputed_deals INT NOT NULL DEFAULT 0,
    total_volume NUMERIC(38, 7) NOT NULL DEFAULT 0
);

CREATE TABLE IF NOT EXISTS completions (
    id BIGSERIAL PRIMARY KEY,
    address TEXT NOT NULL,
    escrow_id BIGINT NOT NULL,
    counterparty TEXT NOT NULL,
    amount NUMERIC(38, 7) NOT NULL DEFAULT 0,
    completed_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    had_dispute BOOLEAN NOT NULL DEFAULT FALSE
);

CREATE INDEX idx_completions_address ON completions(address);
CREATE INDEX idx_completions_completed ON completions(completed_at DESC);
