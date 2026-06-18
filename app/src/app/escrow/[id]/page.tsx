"use client"

import { useParams, useRouter } from "next/navigation"
import { useEffect, useState } from "react"
import { getEscrow } from "@/lib/api"
import { EscrowStatusBadge } from "@/components/StatusBadge"
import { MilestoneList } from "@/components/MilestoneList"
import type { Escrow, Milestone } from "@/types"

export default function EscrowDetailPage() {
  const { id } = useParams<{ id: string }>()
  const router = useRouter()
  const [escrow, setEscrow] = useState<Escrow | null>(null)
  const [milestones, setMilestones] = useState<Milestone[]>([])
  const [loading, setLoading] = useState(true)

  useEffect(() => {
    if (!id) return
    setLoading(true)
    getEscrow(Number(id))
      .then((data) => {
        if (!data) {
          router.push("/escrow")
          return
        }
        setEscrow(data.escrow)
        setMilestones(data.milestones)
      })
      .finally(() => setLoading(false))
  }, [id, router])

  if (loading) {
    return <div className="text-center py-12 text-gray-500">Loading...</div>
  }

  if (!escrow) return null

  return (
    <div className="max-w-3xl mx-auto">
      <button
        onClick={() => router.push("/escrow")}
        className="text-sm text-trust-600 hover:text-trust-700 mb-4 inline-block"
      >
        &larr; Back to Escrows
      </button>

      <div className="bg-white rounded-xl border border-gray-200 p-6 mb-6">
        <div className="flex items-center justify-between mb-4">
          <h1 className="text-xl font-bold">
            Escrow #{escrow.contract_escrow_id}
          </h1>
          <EscrowStatusBadge status={escrow.status} />
        </div>

        <div className="grid grid-cols-2 gap-4 text-sm">
          <div>
            <span className="text-gray-500 block">Client</span>
            <span className="font-mono">{escrow.client_address}</span>
          </div>
          <div>
            <span className="text-gray-500 block">Freelancer</span>
            <span className="font-mono">{escrow.freelancer_address}</span>
          </div>
          <div>
            <span className="text-gray-500 block">Total Amount</span>
            <span className="font-semibold">
              {escrow.total_amount.toLocaleString()}
            </span>
          </div>
          <div>
            <span className="text-gray-500 block">Released</span>
            <span>{escrow.released_amount.toLocaleString()}</span>
          </div>
          <div>
            <span className="text-gray-500 block">Expiry Ledger</span>
            <span>{escrow.expiry_ledger}</span>
          </div>
          <div>
            <span className="text-gray-500 block">Created</span>
            <span>{new Date(escrow.created_at).toLocaleDateString()}</span>
          </div>
        </div>
      </div>

      <MilestoneList
        milestones={milestones}
        escrowId={escrow.contract_escrow_id}
        status={escrow.status}
        clientAddress={escrow.client_address}
      />
    </div>
  )
}
