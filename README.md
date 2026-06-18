# TrustLink

Decentralized B2B escrow platform on Stellar Soroban.

## Architecture

```
TRUSTLINK/
├── contracts/trustlink/   Soroban smart contract (Rust)
├── indexer/               Axum REST + WebSocket + event poller (Rust)
├── app/                   Next.js 14 frontend dashboard (TypeScript)
├── sdk/                   TypeScript SDK (Node.js/browser)
└── docs/                  Documentation
```

## Quick Start

### Smart Contract

```bash
cd contracts/trustlink
cargo test           # 16 tests
```

### Indexer

```bash
cd indexer
cp .env.example .env  # set DATABASE_URL, SOROBAN_RPC_URL
cargo run
```

### Frontend

```bash
cd app
cp .env.example .env.local
npm install && npm run dev
```

### SDK

```bash
cd sdk
npm install && npm run build
```

## Contract

A single `TrustLink` contract with three domains:

| Domain | Methods |
|--------|---------|
| **Escrow** | `create_escrow`, `fund_escrow`, `approve_milestone`, `release_milestone`, `refund_escrow` |
| **Dispute** | `raise_dispute`, `submit_evidence`, `resolve_dispute` |
| **Reputation** | `record_completion`, `get_reputation`, `get_completion_history` |

## SDK Usage

```typescript
import { TrustLinkSDK } from "@trustlink/sdk"

const sdk = new TrustLinkSDK({
  rpcUrl: "https://soroban-testnet.stellar.org",
  networkPassphrase: "Test SDF Future Network ; October 2022",
  contractId: "CCY6...",
})

// Read
const escrow = await sdk.getEscrowDetails(1)
const rep = await sdk.getReputation("G...")

// Write (via Freighter)
const txHash = await sdk.createEscrow(
  "G...client", "G...freelancer", "G...token",
  1000_000000n,       // 1000 USDC
  [{ percentage: 5000, description: "First milestone" }],
  200000,             // expiry ledger
  "G...signer",
  mySignFn
)
```
