# 🤝 TrustLink: Decentralized B2B Escrow on Stellar Soroban

![Stellar](https://img.shields.io/badge/Network-Stellar-black?style=for-the-badge&logo=stellar)
![Soroban](<https://img.shields.io/badge/Contracts-Soroban%20(Rust)-orange?style=for-the-badge>)
![License](https://img.shields.io/badge/License-MIT-green?style=for-the-badge)

**TrustLink** is a decentralized escrow and reputation protocol designed to secure B2B service agreements across borders. By leveraging **Soroban Smart Contracts**, TrustLink eliminates the "Trust Gap" between clients and freelancers, ensuring fair payment even when parties have never met.

This is the main repository containing the smart contract, backend indexer, web dashboard, and TypeScript SDK — all maintained in a monorepo for simpler development and unified deployment.

---

## 🚀 The Mission

To provide a programmable safety net for B2B service transactions. TrustLink ensures that the risk of "paying first" is eliminated, replaced by a secure, neutral vault that only releases funds when milestones are approved.

## 🛠 Features

- **Milestone-Based Escrow:** Secure funds held in Soroban smart contracts, released incrementally as work milestones are approved.
- **Dispute Resolution:** Built-in dispute lifecycle with evidence submission (via SHA-256 hashes) and on-chain mediation verdicts.
- **Reputation System:** On-chain reputation scores tracking total deals, completion rate, dispute history, and total volume for every participant.
- **Real-Time Indexer:** Rust-based indexer polling Soroban RPC for events, exposing a REST + WebSocket API for live dashboard updates.
- **Freighter Wallet:** Seamless integration with the Freighter browser extension for transaction signing.

## 🏗 Technical Stack

- **Frontend:** [Next.js](https://nextjs.org/) 14 (App Router) + Tailwind CSS
- **Smart Contracts:** [Soroban](https://soroban.stellar.org/) (Rust, SDK v22)
- **Backend Indexer:** [Axum](https://github.com/tokio-rs/axum) 0.7 (Rust)
- **Database:** PostgreSQL (via SQLx with migrations)
- **Blockchain:** [Stellar Network](https://www.stellar.org/)
- **Wallet Connection:** [Freighter](https://www.freighter.app/)
- **SDK:** [@stellar/stellar-sdk](https://github.com/stellar/js-stellar-sdk) v12

## 🧪 Local Environments (Folder-Based)

- `contracts/trustlink/` → Soroban smart contract environment
- `indexer/` → Rust/Axum backend indexer environment
- `app/` → Next.js frontend environment
- `sdk/` → TypeScript SDK environment

### Contracts setup

```bash
cd contracts/trustlink
cargo test
```

### Indexer setup

1. `cd indexer`
2. `cp .env.example .env`
3. `cargo run`

### Frontend setup

1. `cd app`
2. `cp .env.example .env.local`
3. `npm install`
4. `npm run dev`

### SDK setup

1. `cd sdk`
2. `npm install`
3. `npm run build`

## 🧪 Required PR CI Gates

TrustLink enforces stack-level CI gates on pull requests through `.github/workflows/ci.yml`.

- **Contracts Required Gate**: `cargo test` in `contracts/trustlink/`
- **Frontend Required Gate**: `npm ci`, `npm run build` in `app/`
- **SDK Required Gate**: `npm ci`, `npm run build` in `sdk/`

### Branch protection setup (GitHub)

For the protected branch (`main`), set these required status checks:

- `Contracts Required Gate`
- `Frontend Required Gate`
- `SDK Required Gate`

## 🔄 How It Works (The TrustLink Flow)

1. **Initiate:** The Client creates an escrow with milestone definitions (percentage weights + descriptions) and deposits funds.
2. **Fund:** The Client funds the escrow via a Stellar token transfer (e.g., USDC).
3. **Deliver:** The Freelancer completes work; the Client approves each milestone as it's delivered.
4. **Release:** Upon approval, funds are released from the contract to the Freelancer for that milestone.
5. **Dispute:** If something goes wrong, either party raises a dispute, submits evidence, and a mediator resolves on-chain.
6. **Reputation:** Upon successful completion, both parties earn reputation points tracked on-chain.

## 🗺 Roadmap

### Phase 1: The Vault (MVP)

- [x] Core Soroban contract: escrow creation, funding, milestone approval, release, refund.
- [x] 16 integration tests covering all escrow scenarios, disputes, and reputation.
- [x] Next.js dashboard with wallet connection, escrow list, and detail views.

### Phase 2: The Indexer

- [x] Rust/Axum event poller consuming Soroban RPC events.
- [x] REST API with 6 endpoints (escrows, milestones, disputes, reputation).
- [x] WebSocket support for real-time dashboard updates.
- [x] PostgreSQL schema (4 migrations) for off-chain metadata.

### Phase 3: SDK & Tooling

- [x] TypeScript SDK with full Soroban transaction builders.
- [x] Freighter integration for browser-based signing.
- [x] Reputation explorer in the web dashboard.

### Phase 4: Mainnet & Scale

- [ ] Deploy to Stellar testnet and mainnet.
- [ ] Public pilot program with regional B2B service providers.
- [ ] Albedo wallet support alongside Freighter.
- [ ] Mobile app (React Native) for on-the-go escrow management.
- [ ] IPFS integration for dispute evidence storage.

## 🔍 Contract Overview

The `TrustLink` contract exposes three domains in a single deployment:

| Domain | Methods | Key Data |
|--------|---------|----------|
| **Escrow** | `create_escrow`, `fund_escrow`, `approve_milestone`, `release_milestone`, `refund_escrow`, `get_escrow_details`, `get_milestone` | Escrow ID, parties, token, amounts, status, milestones |
| **Dispute** | `raise_dispute`, `submit_evidence`, `resolve_dispute`, `get_dispute`, `get_dispute_status` | Dispute ID, escrow ref, reason, evidence hashes, ruling |
| **Reputation** | `record_completion`, `get_reputation`, `get_completion_history` | Total/completed/disputed deals, total volume, paginated history |

## 📦 SDK Usage

```typescript
import { TrustLinkSDK } from "@trustlink/sdk"

const sdk = new TrustLinkSDK({
  rpcUrl: "https://soroban-testnet.stellar.org",
  networkPassphrase: "Test SDF Future Network ; October 2022",
  contractId: "CCY6...",
})

// Read on-chain data
const escrow = await sdk.getEscrowDetails(1)
const rep = await sdk.getReputation("G...")

// Write via Freighter
const txHash = await sdk.createEscrow(
  "G...client",
  "G...freelancer",
  "G...token",
  1000_000000n,
  [{ percentage: 10000, description: "Full delivery" }],
  200000,
  "G...signer",
  signTx
)
```

---

## 🤝 Contributing

TrustLink is an open-source project aimed at building trust in decentralized service economies. We welcome developers, designers, and Stellar ecosystem contributors!

1. Fork the Project.
2. Create your Feature Branch (`git checkout -b feature/NewFeature`).
3. Commit your Changes (`git commit -m 'Add NewFeature'`).
4. Push to the Branch (`git push origin feature/NewFeature`).
5. Open a Pull Request.

## 📄 License

Distributed under the MIT License. See `LICENSE` for more information.
