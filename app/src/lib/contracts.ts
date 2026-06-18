import { signTransaction } from "./freighter"

const CONTRACT_ID = process.env.NEXT_PUBLIC_CONTRACT_ID || ""
const RPC_URL = process.env.NEXT_PUBLIC_SOROBAN_RPC_URL || ""
const PASSPHRASE =
  process.env.NEXT_PUBLIC_NETWORK_PASSPHRASE ||
  "Test SDF Future Network ; October 2022"

export async function buildAndSignTx(
  method: string,
  args: Record<string, unknown>,
  source: string
): Promise<string | null> {
  const sym = (s: string) => Symbol.for(s)

  try {
    const response = await fetch(RPC_URL, {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({
        jsonrpc: "2.0",
        id: 1,
        method: "getTransaction",
        params: {},
      }),
    })
    const sequence = await response
      .json()
      .then((r) => r.result?.sequence || 0)

    const xdr = simulateContractCall(method, args, source, sequence)
    const signed = await signTransaction(xdr, PASSPHRASE)
    return signed
  } catch (err) {
    console.error("Failed to build transaction:", err)
    return null
  }
}

function simulateContractCall(
  method: string,
  _args: Record<string, unknown>,
  source: string,
  _sequence: number
): string {
  return JSON.stringify({
    method,
    source,
    network: PASSPHRASE,
    contractId: CONTRACT_ID,
  })
}

export async function createEscrow(
  client: string,
  freelancer: string,
  token: string,
  amount: string,
  milestones: { percentage: number; description: string }[]
): Promise<string | null> {
  return buildAndSignTx(
    "create_escrow",
    { client, freelancer, token, amount, milestones },
    client
  )
}

export async function fundEscrow(
  escrowId: number,
  funder: string
): Promise<string | null> {
  return buildAndSignTx("fund_escrow", { escrow_id: escrowId }, funder)
}

export async function approveMilestone(
  escrowId: number,
  milestoneIndex: number,
  client: string
): Promise<string | null> {
  return buildAndSignTx(
    "approve_milestone",
    { escrow_id: escrowId, milestone_index: milestoneIndex },
    client
  )
}

export async function raiseDispute(
  escrowId: number,
  raisedBy: string,
  reason: string,
  evidenceHash: string
): Promise<string | null> {
  return buildAndSignTx(
    "raise_dispute",
    { escrow_id: escrowId, raised_by: raisedBy, reason, evidence_hash: evidenceHash },
    raisedBy
  )
}
