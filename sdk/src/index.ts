import {
  SorobanRpc,
  nativeToScVal,
  scValToNative,
  Address,
  Contract,
  xdr,
  TransactionBuilder,
  BASE_FEE,
  Keypair,
} from "@stellar/stellar-sdk"

import type {
  EscrowData,
  MilestoneConfig,
  DisputeData,
  ReputationScore,
  CompletionRecordData,
  SDKConfig,
} from "./types"
import { EscrowStatus } from "./types"

export type {
  EscrowData,
  MilestoneConfig,
  DisputeData,
  ReputationScore,
  CompletionRecordData,
  SDKConfig,
}
export { EscrowStatus }

function parseEscrowStatus(scv: xdr.ScVal): EscrowStatus {
  const native = scValToNative(scv) as [string, null]
  const map: Record<string, EscrowStatus> = {
    Active: EscrowStatus.Active,
    Funded: EscrowStatus.Funded,
    Released: EscrowStatus.Released,
    Disputed: EscrowStatus.Disputed,
    Refunded: EscrowStatus.Refunded,
  }
  return map[native[0]]
}

function mapFromScv(scv: xdr.ScVal): Record<string, xdr.ScVal> {
  const entries = scv.value() as xdr.ScMapEntry[]
  const m: Record<string, xdr.ScVal> = {}
  for (const e of entries) {
    m[e.key().value() as string] = e.val()
  }
  return m
}

function escrowFromScv(scv: xdr.ScVal): EscrowData {
  const m = mapFromScv(scv)
  return {
    id: Number(scValToNative(m.id)),
    client: scValToNative(m.client) as string,
    freelancer: scValToNative(m.freelancer) as string,
    token: scValToNative(m.token) as string,
    total_amount: String(scValToNative(m.total_amount) as bigint),
    released_amount: String(scValToNative(m.released_amount) as bigint),
    status: parseEscrowStatus(m.status),
    expiry_ledger: Number(scValToNative(m.expiry_ledger)),
    milestone_count: Number(scValToNative(m.milestone_count)),
  }
}

function milestoneFromScv(scv: xdr.ScVal): MilestoneConfig {
  const m = mapFromScv(scv)
  return {
    percentage: Number(scValToNative(m.percentage)),
    description: scValToNative(m.description) as string,
    is_approved: scValToNative(m.is_approved) as boolean,
    is_released: scValToNative(m.is_released) as boolean,
  }
}

function disputeFromScv(scv: xdr.ScVal): DisputeData {
  const m = mapFromScv(scv)

  const nativeStatus = scValToNative(m.status) as [string, null]
  const statusMap: Record<string, any> = {
    Open: "Open",
    UnderReview: "UnderReview",
    ResolvedForClient: "ResolvedForClient",
    ResolvedForFreelancer: "ResolvedForFreelancer",
  }

  const rawHashes = scValToNative(m.evidence_hashes) as string[]

  return {
    id: Number(scValToNative(m.id)),
    escrow_id: Number(scValToNative(m.escrow_id)),
    raised_by: scValToNative(m.raised_by) as string,
    reason: scValToNative(m.reason) as string,
    status: statusMap[nativeStatus[0]],
    evidence_hashes: rawHashes,
    verdict_ledger: Number(scValToNative(m.verdict_ledger)),
  }
}

function reputationFromScv(scv: xdr.ScVal): ReputationScore {
  const m = mapFromScv(scv)
  return {
    total_deals: Number(scValToNative(m.total_deals)),
    completed_deals: Number(scValToNative(m.completed_deals)),
    disputed_deals: Number(scValToNative(m.disputed_deals)),
    total_volume: String(scValToNative(m.total_volume) as bigint),
  }
}

function completionFromScv(scv: xdr.ScVal): CompletionRecordData {
  const m = mapFromScv(scv)
  return {
    escrow_id: Number(scValToNative(m.escrow_id)),
    counterparty: scValToNative(m.counterparty) as string,
    amount: String(scValToNative(m.amount) as bigint),
    completed_at: Number(scValToNative(m.completed_at)),
    had_dispute: scValToNative(m.had_dispute) as boolean,
  }
}

export class TrustLinkSDK {
  private server: SorobanRpc.Server
  private contract: Contract
  private config: SDKConfig
  private adminKeypair?: Keypair

  constructor(config: SDKConfig, adminSecret?: string) {
    this.config = config
    this.server = new SorobanRpc.Server(config.rpcUrl, {
      allowHttp: config.allowHttp ?? false,
    })
    this.contract = new Contract(config.contractId)
    if (adminSecret) {
      this.adminKeypair = Keypair.fromSecret(adminSecret)
    }
  }

  async simulate<T = xdr.ScVal>(
    method: string,
    args: xdr.ScVal[]
  ): Promise<T> {
    const source = this.adminKeypair
      ? this.adminKeypair.publicKey()
      : "GAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAWHF"

    const account = await this.server.getAccount(source)
    const tx = new TransactionBuilder(account, {
      fee: BASE_FEE,
      networkPassphrase: this.config.networkPassphrase,
    })
      .addOperation(this.contract.call(method, ...args))
      .setTimeout(300)
      .build()

    const result = await this.server.simulateTransaction(tx)

    if (SorobanRpc.Api.isSimulationError(result)) {
      throw new Error(
        `Simulation failed for ${method}: ${result.error}`
      )
    }

    if (!result.result) {
      throw new Error(
        `Simulation returned no result for ${method}`
      )
    }

    return result.result.retval as T
  }

  async sendWithFreighter(
    method: string,
    args: xdr.ScVal[],
    source: string,
    signTransaction: (xdr: string, passphrase: string) => Promise<string | null>
  ): Promise<string> {
    const account = await this.server.getAccount(source)
    const tx = new TransactionBuilder(account, {
      fee: BASE_FEE,
      networkPassphrase: this.config.networkPassphrase,
    })
      .addOperation(this.contract.call(method, ...args))
      .setTimeout(300)
      .build()

    const signedXdr = await signTransaction(
      tx.toEnvelope().toXDR("base64"),
      this.config.networkPassphrase
    )

    if (!signedXdr) throw new Error("User rejected signing")

    const restored = TransactionBuilder.fromXDR(
      signedXdr,
      this.config.networkPassphrase
    )
    const result = await this.server.sendTransaction(restored)

    if (result.errorResult) {
      throw new Error("Transaction failed")
    }

    return result.hash
  }

  async sendWithAdmin(method: string, args: xdr.ScVal[]): Promise<string> {
    if (!this.adminKeypair) {
      throw new Error("Admin keypair not configured")
    }
    const source = this.adminKeypair.publicKey()
    const account = await this.server.getAccount(source)
    const tx = new TransactionBuilder(account, {
      fee: BASE_FEE,
      networkPassphrase: this.config.networkPassphrase,
    })
      .addOperation(this.contract.call(method, ...args))
      .setTimeout(300)
      .build()

    ;(tx.sign as unknown as (k: Keypair) => void)(this.adminKeypair)
    const result = await this.server.sendTransaction(tx)

    if (result.errorResult) {
      throw new Error("Transaction failed")
    }

    return result.hash
  }

  // ── Escrow ──────────────────────────────────────────────────────────────

  createEscrow(
    client: string,
    freelancer: string,
    token: string,
    totalAmount: bigint,
    milestones: { percentage: number; description: string }[],
    expiryLedger: number,
    signer: string,
    signTx: (xdr: string, passphrase: string) => Promise<string | null>
  ): Promise<string> {
    const msArgs = milestones.map((m) =>
      xdr.ScVal.scvMap([
        new xdr.ScMapEntry({
          key: xdr.ScVal.scvSymbol("percentage"),
          val: nativeToScVal(m.percentage, { type: "u32" }),
        }),
        new xdr.ScMapEntry({
          key: xdr.ScVal.scvSymbol("description"),
          val: nativeToScVal(m.description),
        }),
        new xdr.ScMapEntry({
          key: xdr.ScVal.scvSymbol("is_approved"),
          val: xdr.ScVal.scvBool(false),
        }),
        new xdr.ScMapEntry({
          key: xdr.ScVal.scvSymbol("is_released"),
          val: xdr.ScVal.scvBool(false),
        }),
      ])
    )

    return this.sendWithFreighter(
      "create_escrow",
      [
        Address.fromString(client).toScVal(),
        Address.fromString(freelancer).toScVal(),
        Address.fromString(token).toScVal(),
        nativeToScVal(totalAmount, { type: "i128" }),
        xdr.ScVal.scvVec(msArgs),
        nativeToScVal(expiryLedger, { type: "u32" }),
      ],
      signer,
      signTx
    )
  }

  fundEscrow(
    funder: string,
    escrowId: number,
    signTx: (xdr: string, passphrase: string) => Promise<string | null>
  ): Promise<string> {
    return this.sendWithFreighter(
      "fund_escrow",
      [
        Address.fromString(funder).toScVal(),
        nativeToScVal(escrowId, { type: "u64" }),
      ],
      funder,
      signTx
    )
  }

  approveMilestone(
    escrowId: number,
    milestoneIndex: number,
    client: string,
    signTx: (xdr: string, passphrase: string) => Promise<string | null>
  ): Promise<string> {
    return this.sendWithFreighter(
      "approve_milestone",
      [
        nativeToScVal(escrowId, { type: "u64" }),
        nativeToScVal(milestoneIndex, { type: "u32" }),
      ],
      client,
      signTx
    )
  }

  releaseMilestone(
    escrowId: number,
    milestoneIndex: number,
    signer: string,
    signTx: (xdr: string, passphrase: string) => Promise<string | null>
  ): Promise<string> {
    return this.sendWithFreighter(
      "release_milestone",
      [
        nativeToScVal(escrowId, { type: "u64" }),
        nativeToScVal(milestoneIndex, { type: "u32" }),
      ],
      signer,
      signTx
    )
  }

  raiseDispute(
    escrowId: number,
    raisedBy: string,
    reason: string,
    evidenceHash: string,
    signTx: (xdr: string, passphrase: string) => Promise<string | null>
  ): Promise<string> {
    const hashNoPrefix = evidenceHash.startsWith("0x")
      ? evidenceHash.slice(2)
      : evidenceHash
    const hashBytes = Buffer.from(hashNoPrefix, "hex")

    return this.sendWithFreighter(
      "raise_dispute",
      [
        nativeToScVal(escrowId, { type: "u64" }),
        Address.fromString(raisedBy).toScVal(),
        nativeToScVal(reason),
        xdr.ScVal.scvBytes(hashBytes),
      ],
      raisedBy,
      signTx
    )
  }

  resolveDispute(
    disputeId: number,
    ruling: "ResolvedForClient" | "ResolvedForFreelancer",
    adminSigner: string,
    signTx: (xdr: string, passphrase: string) => Promise<string | null>
  ): Promise<string> {
    const rulingEnum = xdr.ScVal.scvVec([
      xdr.ScVal.scvSymbol(ruling),
      xdr.ScVal.scvVoid(),
    ])

    return this.sendWithFreighter(
      "resolve_dispute",
      [
        nativeToScVal(disputeId, { type: "u64" }),
        rulingEnum,
      ],
      adminSigner,
      signTx
    )
  }

  submitEvidence(
    disputeId: number,
    submitter: string,
    evidenceHash: string,
    signTx: (xdr: string, passphrase: string) => Promise<string | null>
  ): Promise<string> {
    const hashNoPrefix = evidenceHash.startsWith("0x")
      ? evidenceHash.slice(2)
      : evidenceHash
    const hashBytes = Buffer.from(hashNoPrefix, "hex")

    return this.sendWithFreighter(
      "submit_evidence",
      [
        nativeToScVal(disputeId, { type: "u64" }),
        Address.fromString(submitter).toScVal(),
        xdr.ScVal.scvBytes(hashBytes),
      ],
      submitter,
      signTx
    )
  }

  refundEscrow(
    escrowId: number,
    signer: string,
    signTx: (xdr: string, passphrase: string) => Promise<string | null>
  ): Promise<string> {
    return this.sendWithFreighter(
      "refund_escrow",
      [nativeToScVal(escrowId, { type: "u64" })],
      signer,
      signTx
    )
  }

  recordCompletion(
    escrowId: number,
    signer: string,
    signTx: (xdr: string, passphrase: string) => Promise<string | null>
  ): Promise<string> {
    return this.sendWithFreighter(
      "record_completion",
      [nativeToScVal(escrowId, { type: "u64" })],
      signer,
      signTx
    )
  }

  // ── Read-Only ──────────────────────────────────────────────────────────

  async getEscrowDetails(escrowId: number): Promise<EscrowData> {
    const scv = await this.simulate<xdr.ScVal>("get_escrow_details", [
      nativeToScVal(escrowId, { type: "u64" }),
    ])
    return escrowFromScv(scv)
  }

  async getMilestone(
    escrowId: number,
    milestoneIndex: number
  ): Promise<MilestoneConfig> {
    const scv = await this.simulate<xdr.ScVal>("get_milestone", [
      nativeToScVal(escrowId, { type: "u64" }),
      nativeToScVal(milestoneIndex, { type: "u32" }),
    ])
    return milestoneFromScv(scv)
  }

  async getDisputeStatus(disputeId: number): Promise<string> {
    const scv = await this.simulate<xdr.ScVal>("get_dispute_status", [
      nativeToScVal(disputeId, { type: "u64" }),
    ])
    const native = scValToNative(scv) as [string, null]
    return native[0]
  }

  async getDispute(disputeId: number): Promise<DisputeData> {
    const scv = await this.simulate<xdr.ScVal>("get_dispute", [
      nativeToScVal(disputeId, { type: "u64" }),
    ])
    return disputeFromScv(scv)
  }

  async getReputation(addr: string): Promise<ReputationScore> {
    const scv = await this.simulate<xdr.ScVal>("get_reputation", [
      Address.fromString(addr).toScVal(),
    ])
    return reputationFromScv(scv)
  }

  async getCompletionHistory(
    addr: string,
    page: number = 1,
    pageSize: number = 10
  ): Promise<CompletionRecordData[]> {
    const scv = await this.simulate<xdr.ScVal>("get_completion_history", [
      Address.fromString(addr).toScVal(),
      nativeToScVal(page, { type: "u32" }),
      nativeToScVal(pageSize, { type: "u32" }),
    ])
    const vec = scv.value() as xdr.ScVal[]
    return vec.map(completionFromScv)
  }
}

export default TrustLinkSDK
