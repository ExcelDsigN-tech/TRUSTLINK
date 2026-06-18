"use client"

import { useEffect, useState } from "react"
import Link from "next/link"
import { listEscrows } from "@/lib/api"
import { EscrowCard } from "@/components/EscrowCard"
import type { Escrow, EscrowStatus } from "@/types"

const FILTERS: (EscrowStatus | "All")[] = [
  "All",
  "Active",
  "Funded",
  "Disputed",
  "Released",
  "Refunded",
]

export default function EscrowPage() {
  const [escrows, setEscrows] = useState<Escrow[]>([])
  const [filter, setFilter] = useState<EscrowStatus | "All">("All")
  const [loading, setLoading] = useState(true)

  useEffect(() => {
    setLoading(true)
    listEscrows(filter === "All" ? undefined : filter)
      .then(setEscrows)
      .finally(() => setLoading(false))
  }, [filter])

  return (
    <div>
      <div className="flex items-center justify-between mb-6">
        <h1 className="text-2xl font-bold">Escrows</h1>
        <Link
          href="/escrow/new"
          className="bg-trust-600 text-white px-4 py-2 rounded-lg text-sm font-medium hover:bg-trust-700 transition"
        >
          Create Escrow
        </Link>
      </div>

      <div className="flex gap-2 mb-6 overflow-x-auto">
        {FILTERS.map((f) => (
          <button
            key={f}
            onClick={() => setFilter(f)}
            className={`px-3 py-1.5 rounded-lg text-sm font-medium transition ${
              filter === f
                ? "bg-trust-600 text-white"
                : "bg-white border border-gray-200 text-gray-600 hover:bg-gray-50"
            }`}
          >
            {f}
          </button>
        ))}
      </div>

      {loading ? (
        <div className="text-center py-12 text-gray-500">Loading...</div>
      ) : escrows.length === 0 ? (
        <div className="text-center py-12 text-gray-500">
          No escrows found.
        </div>
      ) : (
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
          {escrows.map((e) => (
            <EscrowCard key={e.id} escrow={e} />
          ))}
        </div>
      )}
    </div>
  )
}
