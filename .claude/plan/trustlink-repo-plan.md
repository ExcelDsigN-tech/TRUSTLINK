# Implementation Plan: TrustLink — Decentralized B2B Escrow & Cross-Border Trade Platform

## Overview
Create a **monorepo** under your personal GitHub account `ExcelDsigN-tech` for **TrustLink**: a Stellar-native smart contract platform for B2B escrow, milestone-based payments, and cross-border trade settlement. The project will be structured for the **Drips Wave** program, with scoped issues that contributors can pick up in monthly sprints.

**Why this fits the Wave:** The Stellar Wave already has repos in escrow (Trustless-Work), freelance (Offer Hub), and payroll (Stellopay). TrustLink fills the **B2B trade finance gap** — a natural extension of the ecosystem with clear, independent issues.

---

## Task Type
- [ ] Frontend (→ Gemini)
- [ ] Backend (→ Codex)
- [x] Fullstack (→ Parallel)

---

## Phase 0: Git & GitHub Setup

### Step 0.1 — Initialize Git in REPOFLOW Monorepo
```bash
git init
git add .
git commit -m "chore: initial monorepo scaffold for TrustLink"
```

### Step 0.2 — Create GitHub Repositories
Under `github.com/ExcelDsigN-tech/`:

| Repo | Purpose | Source |
|------|---------|--------|
| `trustlink` | Main monorepo (contracts + app + indexer + sdk + docs) | `C:\Users\pc\drips\REPOFLOW` |
| OR 5 separate repos | `trustlink-contract`, `trustlink-app`, `trustlink-indexer`, `trustlink-sdk`, `trustlink-docs` | Individual subdirs |

**Recommendation:** Single monorepo (`trustlink`) — simpler Wave integration, one repo to approve.

```bash
gh repo create ExcelDsigN-tech/trustlink --public --push --remote origin
```

### Step 0.3 — Add Drips Wave Bot
Install Drips Wave GitHub App on `ExcelDsigN-tech/trustlink` after push.

---

## Phase 1: Complete Soroban Smart Contracts

### Step 1.1 — Escrow Contract
**File:** `repoflow-contract/contracts/trustlink/src/escrow.rs`
**Current state:** `fund_repo()` and `claim_earnings()` are stubs (`unimplemented!()`)

**Replace with full escrow implementation:**

```rust
// Storage keys
pub enum EscrowDataKey {
    Escrow(u64),              // Escrow by ID
    EscrowCount,              // Total escrows created
    EscrowStatus(u64),        // Status by ID: Active, Funded, Released, Disputed, Refunded
    Milestone(u64, u8),       // Milestone by escrow_id + index
}

// Core structs
pub struct Escrow {
    pub id: u64,
    pub client: Address,
    pub freelancer: Address,
    pub token: Address,       // USDC
    pub total_amount: i128,
    pub released_amount: i128,
    pub status: EscrowStatus,
    pub expiry_ledger: u32,
    pub milestone_count: u8,
}

pub struct MilestoneConfig {
    pub percentage: u32,      // Basis points (10000 = 100%)
    pub description: Symbol,
    pub is_approved: bool,
    pub is_released: bool,
}

// Entry points
pub fn create_escrow(client, freelancer, token, amount, milestones, expiry)
pub fn approve_milestone(escrow_id, milestone_index)      // Client auth
pub fn release_milestone(escrow_id, milestone_index)       // Auto after approval
pub fn raise_dispute(escrow_id, reason)
pub fn resolve_dispute(escrow_id, resolution)              // Client+Freelancer both auth
pub fn refund_escrow(escrow_id)                            // After expiry or dispute result
pub fn get_escrow_details(escrow_id) -> Escrow             // View function
```

**Complexity: High (200 Points)**
**Deliverable:** Deploy to Stellar Futurenet, verify with unit tests.

### Step 1.2 — Dispute Resolution Contract
**File:** `repoflow-contract/contracts/trustlink/src/dispute.rs`

```rust
pub struct Dispute {
    pub escrow_id: u64,
    pub raised_by: Address,
    pub reason: Symbol,
    pub status: DisputeStatus,  // Open, UnderReview, ResolvedForClient, ResolvedForFreelancer
    pub evidence_hashes: Vec<BytesN<32>>,
    pub verdict_ledger: u32,
}

pub fn raise_dispute(escrow_id, reason, evidence_hash)
pub fn submit_evidence(dispute_id, evidence_hash)
pub fn resolve_dispute(dispute_id, ruling)       // Two-of-three auth
pub fn get_dispute_status(dispute_id) -> DisputeStatus
```

**Complexity: High (200 Points)**
**Deliverable:** Integrated with Escrow contract, full test coverage.

### Step 1.3 — Reputation Contract
**File:** `repoflow-contract/contracts/trustlink/src/reputation.rs`

```rust
pub fn record_completion(client_addr, freelancer_addr, amount)
pub fn get_reputation(addr) -> ReputationScore
pub fn get_completion_history(addr, page) -> Vec<CompletionRecord>
```

**Complexity: Medium (150 Points)**
**Deliverable:** Immutable on-chain reputation with pagination.

---

## Phase 2: Complete Rust Backend Indexer

### Step 2.1 — Axum Server + REST API
**File:** `repoflow-indexer/src/main.rs` (currently a stub)

- Hook up Axum router with CORS
- Implement poller that reads Soroban contract events
- Implement all 6 endpoints from `API.md`

### Step 2.2 — Event Indexer
- Poll Soroban RPC for new ledgers
- Parse events: `EscrowCreated`, `MilestoneApproved`, `DisputeRaised`, `EscrowResolved`
- Write to PostgreSQL via SQLx

### Step 2.3 — WebSocket Notifications
- Add WebSocket route for real-time escrow status updates
- Push events to connected clients (escrow dashboard)

**Complexity: High (200 Points) each**
**Deliverable:** Running HTTP server + event indexer + WebSocket.

---

## Phase 3: Complete Next.js Frontend

### Step 3.1 — Wallet Connection UI
- Integrate Freighter wallet connect/disconnect
- Display connected Stellar address + balance (USDC/XLM)
- Network switcher (Futurenet → Testnet → Mainnet)

### Step 3.2 — Escrow Dashboard
- List active escrows with status badges
- Create new escrow form (client, freelancer, USDC amount, milestones)
- Milestone management UI (approve, view evidence)
- Real-time status via WebSocket

### Step 3.3 — Dispute Center
- File dispute with reason + evidence upload
- Track dispute status
- View resolved disputes with verdict

### Step 3.4 — Reputation Explorer
- Search by Stellar address
- View completion stats, history, rating

**Complexity: High (200 Points) each**
**Deliverable:** Fully functional Next.js app connected to contracts.

---

## Phase 4: Complete TypeScript SDK

### Step 4.1 — Replace Stub XDR Builders
**File:** `repoflow-sdk/src/client.ts`
- Implement real transaction building using `stellar-sdk`
- `buildCreateEscrowTx()`, `buildApproveMilestoneTx()`, `buildReleaseTx()`
- Submit via Freighter signing callback

### Step 4.2 — SDK Test Suite
- Integration tests with local Soroban sandbox
- Error handling tests
- TypeScript type exports

**Complexity: Medium (150 Points)**  
**Deliverable:** Published SDK package with real XDR builders.

---

## Phase 5: Drips Wave Preparation

### Step 5.1 — Apply Repos to Stellar Wave
1. Go to https://www.drips.network/wave
2. Log in with GitHub (ExcelDsigN-tech)
3. Go to **Maintainers → Orgs and Repos**
4. Install Drips Wave GitHub App on `ExcelDsigN-tech/trustlink`
5. Apply `trustlink` repo to the **Stellar Wave Program**
6. Wait for approval from Wave organizers

### Step 5.2 — Create Wave-Compatible Issues
For each Phase 1-4 task above, create a GitHub issue with:

- **Labels**: `Stellar Wave`, `good first issue`, `high-points` / `medium-points`
- **Description**: Clear scope, acceptance criteria, file references
- **Points**: Explicitly set in Drips dashboard

**Sample Issue Template:**
```markdown
## Title: [CONTRACT] Implement Escrow `create_escrow()` entry point

**Complexity:** High (200 Points)
**Labels:** Stellar Wave, contract

### Scope
Implement the `create_escrow()` function in `contracts/trustlink/src/escrow.rs`.

### Acceptance Criteria
- [ ] Validates client + freelancer addresses
- [ ] Validates milestone percentages sum to 10,000 BPS
- [ ] Stores escrow in persistent storage
- [ ] Emits `EscrowCreated` event
- [ ] Unit tests pass (`cargo test`)

### Files
- `contracts/trustlink/src/escrow.rs`
- `contracts/trustlink/src/test.rs`

### Resources
- Soroban docs: https://stellar.org/soroban
```

### Step 5.3 — Points Budget Estimation

| Phase | Issue | Points | Est. Reward (at $50K pool) |
|-------|-------|--------|---------------------------|
| 1.1 | Escrow contract core | 200 | $200-$500 |
| 1.1 | Escrow unit tests | 150 | $150-$375 |
| 1.2 | Dispute resolution contracts | 200 | $200-$500 |
| 1.3 | Reputation contracts | 150 | $150-$375 |
| 2.1 | Axum HTTP server | 200 | $200-$500 |
| 2.2 | Event indexer | 200 | $200-$500 |
| 2.3 | WebSocket notifications | 150 | $150-$375 |
| 3.1 | Wallet connection UI | 150 | $150-$375 |
| 3.2 | Escrow dashboard | 200 | $200-$500 |
| 3.3 | Dispute center UI | 150 | $150-$375 |
| 3.4 | Reputation explorer | 100 | $100-$250 |
| 4.1 | SDK XDR builders | 150 | $150-$375 |
| 4.2 | SDK test suite | 100 | $100-$250 |
| **Total** | | **2,100** | **$2,100-$5,250** |

---

## Key Files

| File | Operation | Description |
|------|-----------|-------------|
| `repoflow-contract/contracts/trustlink/src/escrow.rs` | Create | Full escrow contract |
| `repoflow-contract/contracts/trustlink/src/dispute.rs` | Create | Dispute resolution contract |
| `repoflow-contract/contracts/trustlink/src/reputation.rs` | Create | Reputation scoring contract |
| `repoflow-contract/contracts/trustlink/src/lib.rs` | Modify | Export new modules |
| `repoflow-contract/contracts/trustlink/src/test.rs` | Create | Integration tests |
| `repoflow-contract/Cargo.toml` | Modify | Add trustlink workspace member |
| `repoflow-indexer/src/main.rs` | Rewrite | Full Axum server |
| `repoflow-indexer/src/poller.rs` | Create | Event poller |
| `repoflow-indexer/src/websocket.rs` | Create | WS notification handler |
| `repoflow-indexer/migrations/0003_create_escrows.sql` | Create | Escrow DB tables |
| `repoflow-indexer/migrations/0004_create_disputes.sql` | Create | Dispute DB tables |
| `repoflow-app/src/app/escrow/page.tsx` | Create | Escrow dashboard page |
| `repoflow-app/src/app/disputes/page.tsx` | Create | Dispute center page |
| `repoflow-app/src/app/reputation/page.tsx` | Create | Reputation explorer page |
| `repoflow-app/src/lib/contracts.ts` | Create | Contract interaction helpers |
| `repoflow-sdk/src/client.ts` | Modify | Real XDR builders |
| `.github/workflows/ci.yml` | Create | CI pipeline |
| `README.md` | Modify | Project docs + Wave badge |

---

## Risks and Mitigation

| Risk | Likelihood | Impact | Mitigation |
|------|-----------|--------|------------|
| Soroban API changes (Protocol 27) | Medium | High | Pin SDK version, monitor stellar.org |
| Escrow contract security bugs | Low | Critical | External audit before mainnet |
| Wave repo application rejected | Low | Medium | Contribute to existing Wave repos first (already have 26K pts) |
| Low contributor engagement | Medium | Low | Set good-first-issues, clear scope, fair points |
| KYC delays for contributors | Low | Medium | Completed upfront by contributor on signup |
| Stellar mainnet fees too high for micro-escrows | Low | Medium | Batch settlements, use USDC only |

---

## Repo Structure (Final)

```
trustlink/
├── .github/
│   └── workflows/
│       ├── ci.yml
│       └── deploy.yml
├── contracts/
│   └── trustlink/
│       ├── Cargo.toml
│       ├── src/
│       │   ├── lib.rs
│       │   ├── escrow.rs
│       │   ├── dispute.rs
│       │   ├── reputation.rs
│       │   └── test.rs
│       └── test_snapshots/
├── app/                          (was repoflow-app)
│   ├── src/
│   │   ├── app/
│   │   ├── lib/
│   │   └── types/
│   ├── package.json
│   └── next.config.mjs
├── indexer/                      (was repoflow-indexer)
│   ├── src/
│   │   ├── main.rs
│   │   ├── poller.rs
│   │   └── websocket.rs
│   ├── migrations/
│   └── API.md
├── sdk/                          (was repoflow-sdk)
│   ├── src/
│   │   ├── client.ts
│   │   └── types.ts
│   └── package.json
├── docs/                         (was repoflow-docs)
│   └── README.md
├── .env.example
├── .gitignore
└── README.md
```

---

## Immediate Next Actions (Today)

1. **Initialize git** in the REPOFLOW directory
2. **Push to GitHub** under `ExcelDsigN-tech/trustlink`
3. **Install Drips Wave GitHub App** on the repo
4. **Apply repo** to Stellar Wave Program
5. **Create first 3 issues** (Escrow contract, tests, event indexer) with Wave labels
6. **Signal availability** in the Stellar Wave Discord / community

---

## User GitHub & Drips Profile

| Account | URL | Notes |
|---------|-----|-------|
| GitHub | https://github.com/ExcelDsigN-tech | Repo owner |
| Drips Wave | https://www.drips.network/wave/users/1a4a6947-8741-4ccc-81e0-8060e6c01ab4 | 26,350 pts, 40+ resolved issues |
| Local project | `C:\Users\pc\drips\REPOFLOW` | 5-component monorepo, not yet in git |

---

## Execution Order (Recommended)

```
Phase 0 (Git → Push → Drips setup)     ← DO THIS FIRST (30 min)
    │
    ▼
Phase 1 (Smart contracts)              ← Core value, Wave-ready
    │
    ▼
Phase 5 (Issues creation)              ← Open for Wave contributors
    │
    ▼
Phase 2 (Backend indexer)              ← Enable frontend
    │
    ▼
Phase 4 (SDK)                          ← Enable integration
    │
    ▼
Phase 3 (Frontend)                     ← User-facing UI last
```

---

**Plan generated and saved to `.claude/plan/trustlink-repo-plan.md`**

**Please review the plan above. You can:**
- **Modify plan**: Tell me what needs adjustment, I'll update the plan
- **Execute plan**: Start with Phase 0 — initialize git, create GitHub repo, apply to Drips Wave
