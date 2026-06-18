#![no_std]
use soroban_sdk::{
    contract, contracterror, contractimpl, contracttype,
    token, Address, BytesN, Env, String, Symbol, Vec,
};

#[cfg(test)]
mod test;

// ── Errors ──────────────────────────────────────────────────────────────────

#[contracterror]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum Error {
    EscrowNotFound = 1,
    Unauthorized = 2,
    InvalidMilestones = 3,
    MilestonesSumExceeds100 = 4,
    EscrowExpired = 5,
    MilestoneAlreadyApproved = 6,
    EscrowNotActive = 7,
    EscrowNotFunded = 8,
    DisputeNotFound = 9,
    DisputeAlreadyResolved = 10,
    DisputeAlreadyOpen = 11,
    InsufficientBalance = 12,
    AlreadyClaimed = 13,
    NoEarnings = 14,
}

// ── Storage Keys ────────────────────────────────────────────────────────────

#[contracttype]
pub enum DataKey {
    Escrow(u64),
    EscrowCount,
    EscrowStatus(u64),
    Milestone(u64, u32),
    MilestoneCount(u64),
    Dispute(u64),
    DisputeCount,
    Reputation(Address),
    CompletionCount(Address),
    CompletionRecord(Address, u32),
}

// ── Data Structures: Escrow ─────────────────────────────────────────────────

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum EscrowStatus {
    Active,
    Funded,
    Released,
    Disputed,
    Refunded,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Escrow {
    pub id: u64,
    pub client: Address,
    pub freelancer: Address,
    pub token: Address,
    pub total_amount: i128,
    pub released_amount: i128,
    pub status: EscrowStatus,
    pub expiry_ledger: u32,
    pub milestone_count: u32,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MilestoneConfig {
    pub percentage: u32,
    pub description: String,
    pub is_approved: bool,
    pub is_released: bool,
}

// ── Data Structures: Dispute ────────────────────────────────────────────────

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DisputeStatus {
    Open,
    UnderReview,
    ResolvedForClient,
    ResolvedForFreelancer,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Dispute {
    pub id: u64,
    pub escrow_id: u64,
    pub raised_by: Address,
    pub reason: String,
    pub status: DisputeStatus,
    pub evidence_hashes: Vec<BytesN<32>>,
    pub verdict_ledger: u32,
}

// ── Data Structures: Reputation ─────────────────────────────────────────────

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReputationScore {
    pub total_deals: u32,
    pub completed_deals: u32,
    pub disputed_deals: u32,
    pub total_volume: i128,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CompletionRecord {
    pub escrow_id: u64,
    pub counterparty: Address,
    pub amount: i128,
    pub completed_at: u64,
    pub had_dispute: bool,
}

// ── Helpers ─────────────────────────────────────────────────────────────────

const MAX_MILESTONES: u32 = 50;
const BASIS_POINTS: u32 = 10_000;

fn escrow_key(id: u64) -> DataKey { DataKey::Escrow(id) }
fn milestone_key(id: u64, idx: u32) -> DataKey { DataKey::Milestone(id, idx) }
fn dispute_key(id: u64) -> DataKey { DataKey::Dispute(id) }
fn rep_key(addr: Address) -> DataKey { DataKey::Reputation(addr) }
fn comp_count_key(addr: Address) -> DataKey { DataKey::CompletionCount(addr) }
fn comp_record_key(addr: Address, idx: u32) -> DataKey { DataKey::CompletionRecord(addr, idx) }

fn save_escrow(env: &Env, escrow: &Escrow) {
    env.storage().persistent().set(&escrow_key(escrow.id), escrow);
}

fn load_escrow(env: &Env, id: u64) -> Result<Escrow, Error> {
    env.storage().persistent().get(&escrow_key(id)).ok_or(Error::EscrowNotFound)
}

fn save_milestone(env: &Env, escrow_id: u64, idx: u32, m: &MilestoneConfig) {
    env.storage().persistent().set(&milestone_key(escrow_id, idx), m);
}

fn load_milestone(env: &Env, escrow_id: u64, idx: u32) -> MilestoneConfig {
    env.storage().persistent().get(&milestone_key(escrow_id, idx)).unwrap()
}

// ── Contract ────────────────────────────────────────────────────────────────

#[contract]
pub struct TrustLink;

#[contractimpl]
impl TrustLink {
    // ═══════════════════════════════════════════════════════════════════════
    // Escrow Entry Points
    // ═══════════════════════════════════════════════════════════════════════

    pub fn create_escrow(
        env: Env,
        client: Address,
        freelancer: Address,
        token: Address,
        total_amount: i128,
        milestones: Vec<MilestoneConfig>,
        expiry_ledger: u32,
    ) -> Result<u64, Error> {
        client.require_auth();

        if total_amount <= 0 {
            return Err(Error::InsufficientBalance);
        }

        if milestones.len() == 0 || milestones.len() > MAX_MILESTONES {
            return Err(Error::InvalidMilestones);
        }

        let sum: u32 = milestones.iter().map(|m| m.percentage).sum();
        if sum != BASIS_POINTS {
            return Err(Error::MilestonesSumExceeds100);
        }

        let count: u64 = env.storage().instance().get(&DataKey::EscrowCount).unwrap_or(0);
        let id = count + 1;

        let escrow = Escrow {
            id,
            client: client.clone(),
            freelancer,
            token,
            total_amount,
            released_amount: 0,
            status: EscrowStatus::Active,
            expiry_ledger,
            milestone_count: milestones.len(),
        };

        save_escrow(&env, &escrow);
        env.storage().instance().set(&DataKey::EscrowCount, &id);

        for (i, milestone) in milestones.iter().enumerate() {
            let idx = i as u32;
            save_milestone(&env, id, idx, &milestone);
        }

        env.events().publish(
            (Symbol::new(&env, "EscrowCreated"), id),
            (client, total_amount),
        );

        Ok(id)
    }

    pub fn fund_escrow(
        env: Env,
        funder: Address,
        escrow_id: u64,
    ) -> Result<(), Error> {
        funder.require_auth();

        let mut escrow = load_escrow(&env, escrow_id)?;

        if escrow.status != EscrowStatus::Active {
            return Err(Error::EscrowNotActive);
        }

        if env.ledger().sequence() >= escrow.expiry_ledger {
            return Err(Error::EscrowExpired);
        }

        let token_client = token::Client::new(&env, &escrow.token);
        token_client.transfer_from(
            &env.current_contract_address(),
            &funder,
            &env.current_contract_address(),
            &escrow.total_amount,
        );

        escrow.status = EscrowStatus::Funded;
        save_escrow(&env, &escrow);

        env.events().publish(
            (Symbol::new(&env, "EscrowFunded"), escrow_id),
            funder,
        );

        Ok(())
    }

    pub fn approve_milestone(
        env: Env,
        escrow_id: u64,
        milestone_index: u32,
    ) -> Result<(), Error> {
        let escrow = load_escrow(&env, escrow_id)?;

        if escrow.status != EscrowStatus::Funded && escrow.status != EscrowStatus::Active {
            return Err(Error::EscrowNotActive);
        }

        escrow.client.require_auth();

        let mut milestone = load_milestone(&env, escrow_id, milestone_index);

        if milestone.is_approved {
            return Err(Error::MilestoneAlreadyApproved);
        }

        if env.ledger().sequence() >= escrow.expiry_ledger {
            return Err(Error::EscrowExpired);
        }

        milestone.is_approved = true;
        save_milestone(&env, escrow_id, milestone_index, &milestone);

        env.events().publish(
            (Symbol::new(&env, "MilestoneApproved"), (escrow_id, milestone_index)),
            (),
        );

        Ok(())
    }

    pub fn release_milestone(
        env: Env,
        escrow_id: u64,
        milestone_index: u32,
    ) -> Result<i128, Error> {
        let mut escrow = load_escrow(&env, escrow_id)?;

        if escrow.status != EscrowStatus::Funded {
            return Err(Error::EscrowNotFunded);
        }

        let milestone = load_milestone(&env, escrow_id, milestone_index);

        if !milestone.is_approved {
            return Err(Error::Unauthorized);
        }

        if milestone.is_released {
            return Err(Error::MilestoneAlreadyApproved);
        }

        if env.ledger().sequence() >= escrow.expiry_ledger {
            return Err(Error::EscrowExpired);
        }

        let release_amount = escrow.total_amount * milestone.percentage as i128 / BASIS_POINTS as i128;

        let token_client = token::Client::new(&env, &escrow.token);
        token_client.transfer(
            &env.current_contract_address(),
            &escrow.freelancer,
            &release_amount,
        );

        escrow.released_amount += release_amount;
        if escrow.released_amount >= escrow.total_amount {
            escrow.status = EscrowStatus::Released;
        }

        let mut updated_milestone = milestone;
        updated_milestone.is_released = true;
        save_milestone(&env, escrow_id, milestone_index, &updated_milestone);
        save_escrow(&env, &escrow);

        env.events().publish(
            (Symbol::new(&env, "MilestoneReleased"), (escrow_id, milestone_index)),
            release_amount,
        );

        Ok(release_amount)
    }

    pub fn raise_dispute(
        env: Env,
        escrow_id: u64,
        raised_by: Address,
        reason: String,
        evidence_hash: BytesN<32>,
    ) -> Result<u64, Error> {
        raised_by.require_auth();

        let mut escrow = load_escrow(&env, escrow_id)?;

        if raised_by != escrow.client && raised_by != escrow.freelancer {
            return Err(Error::Unauthorized);
        }

        if escrow.status == EscrowStatus::Disputed {
            return Err(Error::DisputeAlreadyOpen);
        }

        if escrow.status == EscrowStatus::Released || escrow.status == EscrowStatus::Refunded {
            return Err(Error::EscrowNotActive);
        }

        let count: u64 = env.storage().instance().get(&DataKey::DisputeCount).unwrap_or(0);
        let dispute_id = count + 1;

        let evidence_vec = soroban_sdk::vec![&env, evidence_hash];

        let dispute = Dispute {
            id: dispute_id,
            escrow_id,
            raised_by: raised_by.clone(),
            reason,
            status: DisputeStatus::Open,
            evidence_hashes: evidence_vec,
            verdict_ledger: 0,
        };

        env.storage().persistent().set(&dispute_key(dispute_id), &dispute);
        env.storage().instance().set(&DataKey::DisputeCount, &dispute_id);

        escrow.status = EscrowStatus::Disputed;
        save_escrow(&env, &escrow);

        env.events().publish(
            (Symbol::new(&env, "DisputeRaised"), (escrow_id, dispute_id)),
            raised_by,
        );

        Ok(dispute_id)
    }

    pub fn resolve_dispute(
        env: Env,
        dispute_id: u64,
        ruling: DisputeStatus,
    ) -> Result<(), Error> {
        let mut dispute: Dispute = env.storage().persistent()
            .get(&dispute_key(dispute_id))
            .ok_or(Error::DisputeNotFound)?;

        if dispute.status == DisputeStatus::ResolvedForClient
            || dispute.status == DisputeStatus::ResolvedForFreelancer
        {
            return Err(Error::DisputeAlreadyResolved);
        }

        dispute.status = ruling.clone();
        dispute.verdict_ledger = env.ledger().sequence();

        env.storage().persistent().set(&dispute_key(dispute_id), &dispute);

        let mut escrow = load_escrow(&env, dispute.escrow_id)?;

        match ruling {
            DisputeStatus::ResolvedForFreelancer => {
                let remainder = escrow.total_amount - escrow.released_amount;
                if remainder > 0 {
                    let token_client = token::Client::new(&env, &escrow.token);
                    token_client.transfer(
                        &env.current_contract_address(),
                        &escrow.freelancer,
                        &remainder,
                    );
                    escrow.released_amount = escrow.total_amount;
                }
                escrow.status = EscrowStatus::Released;
            }
            DisputeStatus::ResolvedForClient => {
                let remainder = escrow.total_amount - escrow.released_amount;
                if remainder > 0 {
                    let token_client = token::Client::new(&env, &escrow.token);
                    token_client.transfer(
                        &env.current_contract_address(),
                        &escrow.client,
                        &remainder,
                    );
                }
                escrow.status = EscrowStatus::Refunded;
            }
            _ => {}
        }

        save_escrow(&env, &escrow);

        env.events().publish(
            (Symbol::new(&env, "DisputeResolved"), (dispute.escrow_id, dispute_id)),
            ruling,
        );

        Ok(())
    }

    pub fn refund_escrow(
        env: Env,
        escrow_id: u64,
    ) -> Result<(), Error> {
        let mut escrow = load_escrow(&env, escrow_id)?;

        if escrow.status != EscrowStatus::Funded && escrow.status != EscrowStatus::Active {
            return Err(Error::EscrowNotActive);
        }

        if env.ledger().sequence() < escrow.expiry_ledger {
            return Err(Error::Unauthorized);
        }

        let remainder = escrow.total_amount - escrow.released_amount;
        if remainder > 0 {
            let token_client = token::Client::new(&env, &escrow.token);
            token_client.transfer(
                &env.current_contract_address(),
                &escrow.client,
                &remainder,
            );
        }

        escrow.status = EscrowStatus::Refunded;
        save_escrow(&env, &escrow);

        env.events().publish(
            (Symbol::new(&env, "EscrowRefunded"), escrow_id),
            remainder,
        );

        Ok(())
    }

    pub fn get_escrow_details(env: Env, escrow_id: u64) -> Escrow {
        load_escrow(&env, escrow_id).unwrap()
    }

    pub fn get_milestone(
        env: Env,
        escrow_id: u64,
        milestone_index: u32,
    ) -> MilestoneConfig {
        load_milestone(&env, escrow_id, milestone_index)
    }

    // ═══════════════════════════════════════════════════════════════════════
    // Dispute Entry Points
    // ═══════════════════════════════════════════════════════════════════════

    pub fn submit_evidence(
        env: Env,
        dispute_id: u64,
        submitter: Address,
        evidence_hash: BytesN<32>,
    ) -> Result<(), Error> {
        submitter.require_auth();

        let mut dispute: Dispute = env.storage().persistent()
            .get(&dispute_key(dispute_id))
            .ok_or(Error::DisputeNotFound)?;

        if dispute.status == DisputeStatus::ResolvedForClient
            || dispute.status == DisputeStatus::ResolvedForFreelancer
        {
            return Err(Error::DisputeAlreadyResolved);
        }

        dispute.evidence_hashes.push_back(evidence_hash);
        env.storage().persistent().set(&dispute_key(dispute_id), &dispute);

        Ok(())
    }

    pub fn get_dispute_status(env: Env, dispute_id: u64) -> DisputeStatus {
        let dispute: Dispute = env.storage().persistent()
            .get(&dispute_key(dispute_id))
            .unwrap();
        dispute.status
    }

    pub fn get_dispute(env: Env, dispute_id: u64) -> Dispute {
        env.storage().persistent()
            .get(&dispute_key(dispute_id))
            .unwrap()
    }

    // ═══════════════════════════════════════════════════════════════════════
    // Reputation Entry Points
    // ═══════════════════════════════════════════════════════════════════════

    pub fn record_completion(
        env: Env,
        escrow_id: u64,
    ) -> Result<(), Error> {
        let escrow = load_escrow(&env, escrow_id)?;

        if escrow.status != EscrowStatus::Released {
            return Err(Error::EscrowNotActive);
        }

        for party in [escrow.client.clone(), escrow.freelancer.clone()] {
            let mut score: ReputationScore = env.storage().persistent()
                .get(&rep_key(party.clone()))
                .unwrap_or(ReputationScore {
                    total_deals: 0,
                    completed_deals: 0,
                    disputed_deals: 0,
                    total_volume: 0,
                });

            score.total_deals += 1;
            score.completed_deals += 1;
            score.total_volume += escrow.total_amount;

            env.storage().persistent().set(&rep_key(party.clone()), &score);

            let count: u32 = env.storage().persistent()
                .get(&comp_count_key(party.clone()))
                .unwrap_or(0);
            let new_count = count + 1;

            let record = CompletionRecord {
                escrow_id,
                counterparty: if party == escrow.client {
                    escrow.freelancer.clone()
                } else {
                    escrow.client.clone()
                },
                amount: escrow.total_amount,
                completed_at: env.ledger().timestamp(),
                had_dispute: false,
            };

            env.storage().persistent()
                .set(&comp_record_key(party.clone(), new_count), &record);
            env.storage().persistent()
                .set(&comp_count_key(party), &new_count);
        }

        Ok(())
    }

    pub fn get_reputation(env: Env, addr: Address) -> ReputationScore {
        env.storage().persistent()
            .get(&rep_key(addr))
            .unwrap_or(ReputationScore {
                total_deals: 0,
                completed_deals: 0,
                disputed_deals: 0,
                total_volume: 0,
            })
    }

    pub fn get_completion_history(
        env: Env,
        addr: Address,
        page: u32,
        page_size: u32,
    ) -> Vec<CompletionRecord> {
        let count: u32 = env.storage().persistent()
            .get(&comp_count_key(addr.clone()))
            .unwrap_or(0);

        if count == 0 {
            return Vec::new(&env);
        }

        let start = (page.saturating_sub(1)) * page_size + 1;
        let end = (start + page_size - 1).min(count);

        if start > count {
            return Vec::new(&env);
        }

        let mut results = Vec::new(&env);
        for i in start..=end {
            if let Some(record) = env.storage().persistent()
                .get::<_, CompletionRecord>(&comp_record_key(addr.clone(), i))
            {
                results.push_back(record);
            }
        }
        results
    }
}
