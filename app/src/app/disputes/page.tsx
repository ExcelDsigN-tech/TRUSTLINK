"use client"

import { useEffect, useState } from "react"
import Link from "next/link"
import { listEscrows, getEscrow } from "@/lib/api"
import { DisputeStatusBadge } from "@/components/StatusBadge"
import type { Escrow } from "@/types"

export default function DisputesPage() {
  const [disputed, setDisputed] = useState<Escrow[]>([])
  const [loading, setLoading] = useState(true)

  useEffect(() => {
    setLoading(true)
    listEscrows("Disputed")
      .then(setDisputed)
      .finally(() => setLoading(false))
  }, [])

  return (
    <div>
      <h1 className="text-2xl font-bold mb-6">Dispute Center</h1>

      {loading ? (
        <div className="text-center py-12 text-gray-500">Loading...</div>
      ) : disputed.length === 0 ? (
        <div className="bg-white rounded-xl border border-gray-200 p-12 text-center">
          <p className="text-gray-500 text-lg mb-2">No active disputes</p>
          <p className="text-sm text-gray-400">
            Disputed escrows will appear here.
          </p>
        </div>
      ) : (
        <div className="space-y-3">
          {disputed.map((e) => (
            <Link
              key={e.id}
              href={`/escrow/${e.contract_escrow_id}`}
              className="block bg-white rounded-xl border border-gray-200 p-4 hover:shadow-md transition"
            >
              <div className="flex items-center justify-between">
                <div>
                  <span className="font-medium">
                    Escrow #{e.contract_escrow_id}
                  </span>
                  <p className="text-sm text-gray-500 mt-1">
                    {e.client_address.slice(0, 8)}... →{" "}
                    {e.freelancer_address.slice(0, 8)}...
                  </p>
                </div>
                <div className="text-right">
                  <DisputeStatusBadge status="Open" />
                  <p className="text-sm text-gray-500 mt-1">
                    {e.total_amount.toLocaleString()} USDC
                  </p>
                </div>
              </div>
            </Link>
          ))}
        </div>
      )}
    </div>
  )
}
