import type {
  Escrow,
  Milestone,
  Dispute,
  Reputation,
  CompletionRecord,
} from "@/types"

const API_URL = process.env.NEXT_PUBLIC_API_URL || "http://localhost:3001"

async function fetchJson<T>(url: string): Promise<T> {
  const res = await fetch(url)
  if (!res.ok) throw new Error(`API error: ${res.status}`)
  return res.json()
}

export async function listEscrows(
  status?: string,
  limit = 20,
  offset = 0
): Promise<Escrow[]> {
  const params = new URLSearchParams({ limit: String(limit), offset: String(offset) })
  if (status) params.set("status", status)
  const res = await fetchJson<{ success: boolean; data: Escrow[] }>(
    `${API_URL}/api/escrows?${params}`
  )
  return res.data
}

export async function getEscrow(id: number): Promise<{
  escrow: Escrow
  milestones: Milestone[]
} | null> {
  const res = await fetchJson<{
    success: boolean
    data: Escrow | null
    milestones: Milestone[]
  }>(`${API_URL}/api/escrows/${id}`)
  if (!res.data) return null
  return { escrow: res.data, milestones: res.milestones }
}

export async function getEscrowMilestones(id: number): Promise<Milestone[]> {
  return fetchJson<Milestone[]>(`${API_URL}/api/escrows/${id}/milestones`)
}

export async function getDispute(id: number): Promise<Dispute | null> {
  const res = await fetchJson<{ success: boolean; data: Dispute | null }>(
    `${API_URL}/api/disputes/${id}`
  )
  return res.data
}

export async function getReputation(address: string): Promise<Reputation | null> {
  const res = await fetchJson<{ success: boolean; data: Reputation | null }>(
    `${API_URL}/api/reputation/${address}`
  )
  return res.data
}

export async function getCompletionHistory(
  address: string,
  limit = 20,
  offset = 0
): Promise<CompletionRecord[]> {
  const res = await fetchJson<{ success: boolean; data: CompletionRecord[] }>(
    `${API_URL}/api/reputation/${address}/history?limit=${limit}&offset=${offset}`
  )
  return res.data
}
