export interface Escrow {
  id: number
  contract_escrow_id: number
  client_address: string
  freelancer_address: string
  token_address: string
  total_amount: number
  released_amount: number
  status: EscrowStatus
  expiry_ledger: number
  milestone_count: number
  tx_hash: string | null
  created_at: string
  updated_at: string
}

export type EscrowStatus =
  | "Active"
  | "Funded"
  | "Released"
  | "Disputed"
  | "Refunded"

export interface Milestone {
  id: number
  escrow_id: number
  milestone_index: number
  percentage: number
  description: string
  is_approved: boolean
  is_released: boolean
}

export interface Dispute {
  id: number
  contract_dispute_id: number
  escrow_id: number
  raised_by: string
  reason: string
  status: DisputeStatus
  evidence_hashes: string[]
  verdict_ledger: number
  created_at: string
  updated_at: string
}

export type DisputeStatus =
  | "Open"
  | "UnderReview"
  | "ResolvedForClient"
  | "ResolvedForFreelancer"

export interface Reputation {
  address: string
  total_deals: number
  completed_deals: number
  disputed_deals: number
  total_volume: number
}

export interface CompletionRecord {
  id: number
  address: string
  escrow_id: number
  counterparty: string
  amount: number
  completed_at: string
  had_dispute: boolean
}

export interface ApiResponse<T> {
  success: boolean
  data?: T
  error?: string
}

export interface WalletState {
  address: string | null
  isConnected: boolean
  isFreighter: boolean
}

export interface EscrowFormData {
  client: string
  freelancer: string
  token: string
  totalAmount: string
  milestones: { percentage: number; description: string }[]
  expiryLedger: number
}

export const STATUS_COLORS: Record<EscrowStatus, string> = {
  Active: "bg-blue-100 text-blue-800",
  Funded: "bg-green-100 text-green-800",
  Released: "bg-gray-100 text-gray-800",
  Disputed: "bg-red-100 text-red-800",
  Refunded: "bg-yellow-100 text-yellow-800",
}

export const DISPUTE_STATUS_COLORS: Record<DisputeStatus, string> = {
  Open: "bg-red-100 text-red-800",
  UnderReview: "bg-yellow-100 text-yellow-800",
  ResolvedForClient: "bg-green-100 text-green-800",
  ResolvedForFreelancer: "bg-green-100 text-green-800",
}
