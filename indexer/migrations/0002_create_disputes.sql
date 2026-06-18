CREATE TABLE IF NOT EXISTS disputes (
    id BIGSERIAL PRIMARY KEY,
    contract_dispute_id BIGINT NOT NULL UNIQUE,
    escrow_id BIGINT NOT NULL,
    raised_by TEXT NOT NULL,
    reason TEXT NOT NULL DEFAULT '',
    status TEXT NOT NULL DEFAULT 'Open',
    evidence_hashes TEXT[] NOT NULL DEFAULT '{}',
    verdict_ledger BIGINT NOT NULL DEFAULT 0,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_disputes_escrow ON disputes(escrow_id);
CREATE INDEX idx_disputes_status ON disputes(status);
