import { STATUS_COLORS, DISPUTE_STATUS_COLORS } from "@/types"
import type { EscrowStatus, DisputeStatus } from "@/types"

export function EscrowStatusBadge({ status }: { status: EscrowStatus }) {
  return (
    <span
      className={`inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium ${STATUS_COLORS[status] || "bg-gray-100 text-gray-800"}`}
    >
      {status}
    </span>
  )
}

export function DisputeStatusBadge({ status }: { status: DisputeStatus }) {
  return (
    <span
      className={`inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium ${DISPUTE_STATUS_COLORS[status] || "bg-gray-100 text-gray-800"}`}
    >
      {status.replace(/([A-Z])/g, " $1").trim()}
    </span>
  )
}
