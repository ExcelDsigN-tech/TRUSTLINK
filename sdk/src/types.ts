export enum EscrowStatus {
  Active = "Active",
  Funded = "Funded",
  Released = "Released",
  Disputed = "Disputed",
  Refunded = "Refunded",
}

export enum DisputeStatus {
  Open = "Open",
  UnderReview = "UnderReview",
  ResolvedForClient = "ResolvedForClient",
  ResolvedForFreelancer = "ResolvedForFreelancer",
}

export interface EscrowData {
  id: number
  client: string
  freelancer: string
  token: string
  total_amount: string
  released_amount: string
  status: EscrowStatus
  expiry_ledger: number
  milestone_count: number
}

export interface MilestoneConfig {
  percentage: number
  description: string
  is_approved: boolean
  is_released: boolean
}

export interface DisputeData {
  id: number
  escrow_id: number
  raised_by: string
  reason: string
  status: DisputeStatus
  evidence_hashes: string[]
  verdict_ledger: number
}

export interface ReputationScore {
  total_deals: number
  completed_deals: number
  disputed_deals: number
  total_volume: string
}

export interface CompletionRecordData {
  escrow_id: number
  counterparty: string
  amount: string
  completed_at: number
  had_dispute: boolean
}

export interface SDKConfig {
  rpcUrl: string
  networkPassphrase: string
  contractId: string
  allowHttp?: boolean
}
