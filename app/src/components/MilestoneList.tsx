import type { Milestone } from "@/types"
import { approveMilestone } from "@/lib/contracts"

export function MilestoneList({
  milestones,
  escrowId,
  status,
  clientAddress,
}: {
  milestones: Milestone[]
  escrowId: number
  status: string
  clientAddress: string
}) {
  const handleApprove = async (index: number) => {
    if (!confirm(`Approve milestone ${index + 1}?`)) return
    await approveMilestone(escrowId, index, clientAddress)
    window.location.reload()
  }

  const canApprove = status === "Funded" || status === "Active"

  return (
    <div className="space-y-3">
      <h3 className="text-lg font-semibold">Milestones</h3>
      {milestones.map((m) => (
        <div
          key={m.id}
          className="bg-white rounded-lg border border-gray-200 p-4"
        >
          <div className="flex items-center justify-between">
            <div>
              <span className="font-medium">Milestone {m.milestone_index + 1}</span>
              <p className="text-sm text-gray-500">{m.description}</p>
            </div>
            <div className="text-right">
              <span className="text-sm font-medium">{m.percentage / 100}%</span>
              <div className="flex items-center gap-2 mt-1">
                {m.is_released ? (
                  <span className="text-xs text-green-600 font-medium">
                    Released
                  </span>
                ) : m.is_approved ? (
                  <span className="text-xs text-blue-600 font-medium">
                    Approved
                  </span>
                ) : (
                  <button
                    onClick={() => handleApprove(m.milestone_index)}
                    disabled={!canApprove}
                    className="text-xs bg-trust-600 text-white px-3 py-1 rounded hover:bg-trust-700 disabled:opacity-50 disabled:cursor-not-allowed transition"
                  >
                    Approve
                  </button>
                )}
              </div>
            </div>
          </div>
        </div>
      ))}
    </div>
  )
}
