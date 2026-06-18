import Link from "next/link"
import { EscrowStatusBadge } from "./StatusBadge"
import type { Escrow } from "@/types"

export function EscrowCard({ escrow }: { escrow: Escrow }) {
  return (
    <Link
      href={`/escrow/${escrow.contract_escrow_id}`}
      className="block bg-white rounded-xl border border-gray-200 p-5 hover:shadow-md transition"
    >
      <div className="flex items-center justify-between mb-3">
        <span className="text-sm font-mono text-gray-500">
          #{escrow.contract_escrow_id}
        </span>
        <EscrowStatusBadge status={escrow.status} />
      </div>
      <div className="space-y-1 text-sm">
        <div className="flex justify-between">
          <span className="text-gray-500">Client</span>
          <span className="font-mono text-xs">
            {escrow.client_address.slice(0, 8)}...
          </span>
        </div>
        <div className="flex justify-between">
          <span className="text-gray-500">Freelancer</span>
          <span className="font-mono text-xs">
            {escrow.freelancer_address.slice(0, 8)}...
          </span>
        </div>
        <div className="flex justify-between">
          <span className="text-gray-500">Amount</span>
          <span className="font-medium">{escrow.total_amount.toLocaleString()}</span>
        </div>
        <div className="flex justify-between">
          <span className="text-gray-500">Released</span>
          <span>{escrow.released_amount.toLocaleString()}</span>
        </div>
        <div className="flex justify-between">
          <span className="text-gray-500">Milestones</span>
          <span>{escrow.milestone_count}</span>
        </div>
      </div>
    </Link>
  )
}
