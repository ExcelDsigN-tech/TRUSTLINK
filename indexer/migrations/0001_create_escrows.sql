CREATE TABLE IF NOT EXISTS escrows (
    id BIGSERIAL PRIMARY KEY,
    contract_escrow_id BIGINT NOT NULL UNIQUE,
    client_address TEXT NOT NULL,
    freelancer_address TEXT NOT NULL,
    token_address TEXT NOT NULL,
    total_amount NUMERIC(38, 7) NOT NULL DEFAULT 0,
    released_amount NUMERIC(38, 7) NOT NULL DEFAULT 0,
    status TEXT NOT NULL DEFAULT 'Active',
    expiry_ledger BIGINT NOT NULL DEFAULT 0,
    milestone_count INT NOT NULL DEFAULT 0,
    tx_hash TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_escrows_status ON escrows(status);
CREATE INDEX idx_escrows_client ON escrows(client_address);
CREATE INDEX idx_escrows_freelancer ON escrows(freelancer_address);

CREATE TABLE IF NOT EXISTS milestones (
    id BIGSERIAL PRIMARY KEY,
    escrow_id BIGINT NOT NULL REFERENCES escrows(id) ON DELETE CASCADE,
    milestone_index INT NOT NULL,
    percentage INT NOT NULL CHECK (percentage >= 0 AND percentage <= 10000),
    description TEXT NOT NULL DEFAULT '',
    is_approved BOOLEAN NOT NULL DEFAULT FALSE,
    is_released BOOLEAN NOT NULL DEFAULT FALSE,
    UNIQUE(escrow_id, milestone_index)
);

CREATE INDEX idx_milestones_escrow ON milestones(escrow_id);
