#![cfg(test)]
use super::*;
use soroban_sdk::{
    testutils::{Address as _, Events, Ledger},
    token, vec, Address, BytesN, Env, String, Vec,
};

fn make_milestones(env: &Env, percentages: Vec<u32>) -> Vec<MilestoneConfig> {
    let mut result = Vec::new(env);
    for (i, p) in percentages.iter().enumerate() {
        result.push_back(MilestoneConfig {
            percentage: p,
            description: String::from_str(env, match i {
                0 => "First",
                1 => "Second",
                2 => "Third",
                _ => "Milestone",
            }),
            is_approved: false,
            is_released: false,
        });
    }
    result
}

fn default_milestones(env: &Env) -> Vec<MilestoneConfig> {
    make_milestones(env, vec![&env, 5000u32, 3000u32, 2000u32])
}

// ═══════════════════════════════════════════════════════════════════════════
// Escrow Tests
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn test_create_escrow_success() {
    let env = Env::default();
    env.mock_all_auths();
    #[allow(deprecated)]
    let token = env.register_stellar_asset_contract(Address::generate(&env));
    let contract_id = env.register(TrustLink, ());
    let client = TrustLinkClient::new(&env, &contract_id);
    let alice = Address::generate(&env);
    let bob = Address::generate(&env);

    let milestones = default_milestones(&env);
    let id = client.create_escrow(&alice, &bob, &token, &1_000_000, &milestones, &100_000);

    assert_eq!(id, 1);

    let escrow = client.get_escrow_details(&id);
    assert_eq!(escrow.client, alice);
    assert_eq!(escrow.freelancer, bob);
    assert_eq!(escrow.token, token);
    assert_eq!(escrow.total_amount, 1_000_000);
    assert_eq!(escrow.released_amount, 0);
    assert_eq!(escrow.status, EscrowStatus::Active);
    assert_eq!(escrow.milestone_count, 3);

    let m0 = client.get_milestone(&id, &0);
    assert_eq!(m0.percentage, 5000);
    assert_eq!(m0.is_approved, false);
    assert_eq!(m0.is_released, false);
}

#[test]
fn test_create_escrow_invalid_milestones_empty() {
    let env = Env::default();
    env.mock_all_auths();
    #[allow(deprecated)]
    let token = env.register_stellar_asset_contract(Address::generate(&env));
    let contract_id = env.register(TrustLink, ());
    let client = TrustLinkClient::new(&env, &contract_id);
    let alice = Address::generate(&env);
    let bob = Address::generate(&env);

    let result = client.try_create_escrow(
        &alice, &bob, &token, &1_000_000, &Vec::new(&env), &100_000,
    );
    assert_eq!(result.err().unwrap().unwrap(), Error::InvalidMilestones);
}

#[test]
fn test_create_escrow_invalid_sum() {
    let env = Env::default();
    env.mock_all_auths();
    #[allow(deprecated)]
    let token = env.register_stellar_asset_contract(Address::generate(&env));
    let contract_id = env.register(TrustLink, ());
    let client = TrustLinkClient::new(&env, &contract_id);
    let alice = Address::generate(&env);
    let bob = Address::generate(&env);

    let bad_milestones = make_milestones(&env, vec![&env, 3000u32, 3000u32]);
    let result = client.try_create_escrow(
        &alice, &bob, &token, &1_000_000, &bad_milestones, &100_000,
    );
    assert_eq!(result.err().unwrap().unwrap(), Error::MilestonesSumExceeds100);
}

#[test]
fn test_fund_escrow_success() {
    let env = Env::default();
    env.mock_all_auths();
    #[allow(deprecated)]
    let token = env.register_stellar_asset_contract(Address::generate(&env));
    let contract_id = env.register(TrustLink, ());
    let client = TrustLinkClient::new(&env, &contract_id);
    let alice = Address::generate(&env);
    let bob = Address::generate(&env);
    let funder = Address::generate(&env);

    token::StellarAssetClient::new(&env, &token).mint(&funder, &1_000_000);
    token::Client::new(&env, &token).approve(&funder, &contract_id, &1_000_000, &200_000);

    let milestones = default_milestones(&env);
    let id = client.create_escrow(&alice, &bob, &token, &1_000_000, &milestones, &200_000);

    client.fund_escrow(&funder, &id);

    let escrow = client.get_escrow_details(&id);
    assert_eq!(escrow.status, EscrowStatus::Funded);
}

#[test]
fn test_fund_escrow_not_found() {
    let env = Env::default();
    env.mock_all_auths();
    #[allow(deprecated)]
    let token = env.register_stellar_asset_contract(Address::generate(&env));
    let contract_id = env.register(TrustLink, ());
    let client = TrustLinkClient::new(&env, &contract_id);
    let funder = Address::generate(&env);

    token::StellarAssetClient::new(&env, &token).mint(&funder, &1_000_000);
    token::Client::new(&env, &token).approve(&funder, &contract_id, &1_000_000, &200_000);

    let result = client.try_fund_escrow(&funder, &99);
    assert_eq!(result.err().unwrap().unwrap(), Error::EscrowNotFound);
}

#[test]
fn test_approve_and_release_milestone() {
    let env = Env::default();
    env.mock_all_auths();
    #[allow(deprecated)]
    let token = env.register_stellar_asset_contract(Address::generate(&env));
    let contract_id = env.register(TrustLink, ());
    let client = TrustLinkClient::new(&env, &contract_id);
    let alice = Address::generate(&env);
    let bob = Address::generate(&env);
    let funder = Address::generate(&env);

    token::StellarAssetClient::new(&env, &token).mint(&funder, &1_000_000);
    token::Client::new(&env, &token).approve(&funder, &contract_id, &1_000_000, &200_000);

    let milestones = default_milestones(&env);
    let id = client.create_escrow(&alice, &bob, &token, &1_000_000, &milestones, &200_000);
    client.fund_escrow(&funder, &id);

    client.approve_milestone(&id, &0);
    let amount = client.release_milestone(&id, &0);
    assert_eq!(amount, 500_000); // 50% of 1_000_000

    let m0 = client.get_milestone(&id, &0);
    assert!(m0.is_approved);
    assert!(m0.is_released);

    let escrow = client.get_escrow_details(&id);
    assert_eq!(escrow.released_amount, 500_000);
    assert_eq!(escrow.status, EscrowStatus::Funded); // Still funded, not all released

    // Release remaining milestones
    client.approve_milestone(&id, &1);
    client.release_milestone(&id, &1);
    client.approve_milestone(&id, &2);
    client.release_milestone(&id, &2);

    let escrow = client.get_escrow_details(&id);
    assert_eq!(escrow.released_amount, 1_000_000);
    assert_eq!(escrow.status, EscrowStatus::Released);
}

#[test]
fn test_approve_milestone_twice_fails() {
    let env = Env::default();
    env.mock_all_auths();
    #[allow(deprecated)]
    let token = env.register_stellar_asset_contract(Address::generate(&env));
    let contract_id = env.register(TrustLink, ());
    let client = TrustLinkClient::new(&env, &contract_id);
    let alice = Address::generate(&env);
    let bob = Address::generate(&env);
    let funder = Address::generate(&env);

    token::StellarAssetClient::new(&env, &token).mint(&funder, &1_000_000);
    token::Client::new(&env, &token).approve(&funder, &contract_id, &1_000_000, &200_000);

    let milestones = default_milestones(&env);
    let id = client.create_escrow(&alice, &bob, &token, &1_000_000, &milestones, &200_000);
    client.fund_escrow(&funder, &id);

    client.approve_milestone(&id, &0);
    let result = client.try_approve_milestone(&id, &0);
    assert_eq!(result.err().unwrap().unwrap(), Error::MilestoneAlreadyApproved);
}

#[test]
fn test_release_unapproved_milestone_fails() {
    let env = Env::default();
    env.mock_all_auths();
    #[allow(deprecated)]
    let token = env.register_stellar_asset_contract(Address::generate(&env));
    let contract_id = env.register(TrustLink, ());
    let client = TrustLinkClient::new(&env, &contract_id);
    let alice = Address::generate(&env);
    let bob = Address::generate(&env);
    let funder = Address::generate(&env);

    token::StellarAssetClient::new(&env, &token).mint(&funder, &1_000_000);
    token::Client::new(&env, &token).approve(&funder, &contract_id, &1_000_000, &200_000);

    let milestones = default_milestones(&env);
    let id = client.create_escrow(&alice, &bob, &token, &1_000_000, &milestones, &200_000);
    client.fund_escrow(&funder, &id);

    let result = client.try_release_milestone(&id, &0);
    assert_eq!(result.err().unwrap().unwrap(), Error::Unauthorized);
}

// ═══════════════════════════════════════════════════════════════════════════
// Dispute Tests
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn test_raise_and_resolve_dispute_for_freelancer() {
    let env = Env::default();
    env.mock_all_auths();
    #[allow(deprecated)]
    let token = env.register_stellar_asset_contract(Address::generate(&env));
    let contract_id = env.register(TrustLink, ());
    let client = TrustLinkClient::new(&env, &contract_id);
    let alice = Address::generate(&env);
    let bob = Address::generate(&env);
    let funder = Address::generate(&env);

    token::StellarAssetClient::new(&env, &token).mint(&funder, &1_000_000);
    token::Client::new(&env, &token).approve(&funder, &contract_id, &1_000_000, &200_000);

    let milestones = default_milestones(&env);
    let id = client.create_escrow(&alice, &bob, &token, &1_000_000, &milestones, &200_000);
    client.fund_escrow(&funder, &id);

    // Raise dispute
    let evidence = BytesN::from_array(&env, &[1u8; 32]);
    let dispute_id = client.raise_dispute(
        &id, &alice, &String::from_str(&env, "Deliverable not met"), &evidence,
    );
    assert_eq!(dispute_id, 1);

    let escrow = client.get_escrow_details(&id);
    assert_eq!(escrow.status, EscrowStatus::Disputed);

    // Resolve in favor of freelancer
    client.resolve_dispute(&dispute_id, &DisputeStatus::ResolvedForFreelancer);

    let escrow = client.get_escrow_details(&id);
    assert_eq!(escrow.status, EscrowStatus::Released);
    assert_eq!(escrow.released_amount, 1_000_000);

    let status = client.get_dispute_status(&dispute_id);
    assert_eq!(status, DisputeStatus::ResolvedForFreelancer);
}

#[test]
fn test_raise_and_resolve_dispute_for_client() {
    let env = Env::default();
    env.mock_all_auths();
    #[allow(deprecated)]
    let token = env.register_stellar_asset_contract(Address::generate(&env));
    let contract_id = env.register(TrustLink, ());
    let client = TrustLinkClient::new(&env, &contract_id);
    let alice = Address::generate(&env);
    let bob = Address::generate(&env);
    let funder = Address::generate(&env);

    token::StellarAssetClient::new(&env, &token).mint(&funder, &1_000_000);
    token::Client::new(&env, &token).approve(&funder, &contract_id, &1_000_000, &200_000);

    let milestones = default_milestones(&env);
    let id = client.create_escrow(&alice, &bob, &token, &1_000_000, &milestones, &200_000);
    client.fund_escrow(&funder, &id);

    // Client approves and releases milestone 1 first
    client.approve_milestone(&id, &0);
    client.release_milestone(&id, &0);

    let evidence = BytesN::from_array(&env, &[2u8; 32]);
    let dispute_id = client.raise_dispute(
        &id, &alice, &String::from_str(&env, "Quality issues"), &evidence,
    );

    client.resolve_dispute(&dispute_id, &DisputeStatus::ResolvedForClient);

    let escrow = client.get_escrow_details(&id);
    assert_eq!(escrow.status, EscrowStatus::Refunded);
    assert_eq!(escrow.released_amount, 500_000); // Only milestone 1 was released
}

#[test]
fn test_resolve_dispute_twice_fails() {
    let env = Env::default();
    env.mock_all_auths();
    #[allow(deprecated)]
    let token = env.register_stellar_asset_contract(Address::generate(&env));
    let contract_id = env.register(TrustLink, ());
    let client = TrustLinkClient::new(&env, &contract_id);
    let alice = Address::generate(&env);
    let bob = Address::generate(&env);
    let funder = Address::generate(&env);

    token::StellarAssetClient::new(&env, &token).mint(&funder, &1_000_000);
    token::Client::new(&env, &token).approve(&funder, &contract_id, &1_000_000, &200_000);

    let milestones = default_milestones(&env);
    let id = client.create_escrow(&alice, &bob, &token, &1_000_000, &milestones, &200_000);
    client.fund_escrow(&funder, &id);

    let evidence = BytesN::from_array(&env, &[1u8; 32]);
    let dispute_id = client.raise_dispute(
        &id, &alice, &String::from_str(&env, "Dispute"), &evidence,
    );

    client.resolve_dispute(&dispute_id, &DisputeStatus::ResolvedForFreelancer);

    let result = client.try_resolve_dispute(&dispute_id, &DisputeStatus::ResolvedForClient);
    assert_eq!(result.err().unwrap().unwrap(), Error::DisputeAlreadyResolved);
}

#[test]
fn test_submit_evidence() {
    let env = Env::default();
    env.mock_all_auths();
    #[allow(deprecated)]
    let token = env.register_stellar_asset_contract(Address::generate(&env));
    let contract_id = env.register(TrustLink, ());
    let client = TrustLinkClient::new(&env, &contract_id);
    let alice = Address::generate(&env);
    let bob = Address::generate(&env);
    let funder = Address::generate(&env);

    token::StellarAssetClient::new(&env, &token).mint(&funder, &1_000_000);
    token::Client::new(&env, &token).approve(&funder, &contract_id, &1_000_000, &200_000);

    let milestones = default_milestones(&env);
    let id = client.create_escrow(&alice, &bob, &token, &1_000_000, &milestones, &200_000);
    client.fund_escrow(&funder, &id);

    let evidence1 = BytesN::from_array(&env, &[1u8; 32]);
    let dispute_id = client.raise_dispute(
        &id, &alice, &String::from_str(&env, "Dispute"), &evidence1,
    );

    let evidence2 = BytesN::from_array(&env, &[2u8; 32]);
    client.submit_evidence(&dispute_id, &bob, &evidence2);

    let dispute = client.get_dispute(&dispute_id);
    assert_eq!(dispute.evidence_hashes.len(), 2);
}

// ═══════════════════════════════════════════════════════════════════════════
// Refund Tests
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn test_refund_after_expiry() {
    let env = Env::default();
    env.mock_all_auths();
    #[allow(deprecated)]
    let token = env.register_stellar_asset_contract(Address::generate(&env));
    let contract_id = env.register(TrustLink, ());
    let client = TrustLinkClient::new(&env, &contract_id);
    let alice = Address::generate(&env);
    let bob = Address::generate(&env);
    let funder = Address::generate(&env);

    token::StellarAssetClient::new(&env, &token).mint(&funder, &1_000_000);
    token::Client::new(&env, &token).approve(&funder, &contract_id, &1_000_000, &200_000);

    let milestones = default_milestones(&env);
    let id = client.create_escrow(&alice, &bob, &token, &1_000_000, &milestones, &50);
    client.fund_escrow(&funder, &id);

    // Advance ledger past expiry
    env.ledger().set_sequence_number(100);

    client.refund_escrow(&id);

    let escrow = client.get_escrow_details(&id);
    assert_eq!(escrow.status, EscrowStatus::Refunded);
}

// ═══════════════════════════════════════════════════════════════════════════
// Reputation Tests
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn test_reputation_after_completion() {
    let env = Env::default();
    env.mock_all_auths();
    #[allow(deprecated)]
    let token = env.register_stellar_asset_contract(Address::generate(&env));
    let contract_id = env.register(TrustLink, ());
    let client = TrustLinkClient::new(&env, &contract_id);
    let alice = Address::generate(&env);
    let bob = Address::generate(&env);
    let funder = Address::generate(&env);

    token::StellarAssetClient::new(&env, &token).mint(&funder, &1_000_000);
    token::Client::new(&env, &token).approve(&funder, &contract_id, &1_000_000, &200_000);

    let milestones = default_milestones(&env);
    let id = client.create_escrow(&alice, &bob, &token, &1_000_000, &milestones, &200_000);
    client.fund_escrow(&funder, &id);

    // Complete all milestones
    for i in 0..3 {
        client.approve_milestone(&id, &i);
        client.release_milestone(&id, &i);
    }

    env.ledger().set_timestamp(1_700_000_000);
    client.record_completion(&id);

    let rep_a = client.get_reputation(&alice);
    assert_eq!(rep_a.total_deals, 1);
    assert_eq!(rep_a.completed_deals, 1);
    assert_eq!(rep_a.total_volume, 1_000_000);

    let rep_b = client.get_reputation(&bob);
    assert_eq!(rep_b.total_deals, 1);
    assert_eq!(rep_b.completed_deals, 1);

    let history = client.get_completion_history(&alice, &1, &10);
    assert_eq!(history.len(), 1);
    assert_eq!(history.get(0).unwrap().escrow_id, id);
    assert_eq!(history.get(0).unwrap().counterparty, bob);
    assert_eq!(history.get(0).unwrap().amount, 1_000_000);
}

#[test]
fn test_reputation_empty() {
    let env = Env::default();
    env.mock_all_auths();
    let contract_id = env.register(TrustLink, ());
    let client = TrustLinkClient::new(&env, &contract_id);
    let stranger = Address::generate(&env);

    let rep = client.get_reputation(&stranger);
    assert_eq!(rep.total_deals, 0);
    assert_eq!(rep.completed_deals, 0);
    assert_eq!(rep.total_volume, 0);

    let history = client.get_completion_history(&stranger, &1, &10);
    assert_eq!(history.len(), 0);
}

#[test]
fn test_events_emitted() {
    let env = Env::default();
    env.mock_all_auths();
    #[allow(deprecated)]
    let token = env.register_stellar_asset_contract(Address::generate(&env));
    let contract_id = env.register(TrustLink, ());
    let client = TrustLinkClient::new(&env, &contract_id);
    let alice = Address::generate(&env);
    let bob = Address::generate(&env);
    let funder = Address::generate(&env);

    token::StellarAssetClient::new(&env, &token).mint(&funder, &1_000_000);
    token::Client::new(&env, &token).approve(&funder, &contract_id, &1_000_000, &200_000);

    let milestones = default_milestones(&env);
    let id = client.create_escrow(&alice, &bob, &token, &1_000_000, &milestones, &200_000);
    client.fund_escrow(&funder, &id);

    let events = env.events().all();
    // Expect: EscrowCreated + approve event + EscrowFunded
    assert!(events.len() >= 2);

    let (_, _topics, _) = events.get(0).unwrap();
    // Event topics are: (Symbol, id)
    // Just verify we have events without checking exact format
}
