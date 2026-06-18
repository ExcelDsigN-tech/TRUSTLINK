"use client"

import { useState } from "react"
import { Plus, Trash2 } from "lucide-react"
import { createEscrow } from "@/lib/contracts"
import type { WalletState } from "@/types"

interface Props {
  wallet: WalletState | null
  onSuccess: () => void
}

export function CreateEscrowForm({ wallet, onSuccess }: Props) {
  const [freelancer, setFreelancer] = useState("")
  const [amount, setAmount] = useState("")
  const [expiry, setExpiry] = useState("100000")
  const [milestones, setMilestones] = useState([
    { percentage: 10000, description: "Full Payment" },
  ])
  const [submitting, setSubmitting] = useState(false)

  const totalPct = milestones.reduce((s, m) => s + m.percentage, 0)
  const isValid = totalPct === 10000

  const addMilestone = () => {
    if (milestones.length >= 10) return
    setMilestones([...milestones, { percentage: 0, description: "" }])
  }

  const removeMilestone = (i: number) => {
    setMilestones(milestones.filter((_, idx) => idx !== i))
  }

  const updateMilestone = (
    i: number,
    field: keyof (typeof milestones)[0],
    value: string | number
  ) => {
    const updated = [...milestones]
    updated[i] = { ...updated[i], [field]: value }
    setMilestones(updated)
  }

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault()
    if (!wallet?.address || !isValid || submitting) return

    setSubmitting(true)
    try {
      await createEscrow(
        wallet.address,
        freelancer,
        "USDC",
        amount,
        milestones
      )
      onSuccess()
    } catch (err) {
      console.error("Failed to create escrow:", err)
    } finally {
      setSubmitting(false)
    }
  }

  return (
    <form onSubmit={handleSubmit} className="space-y-6">
      <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
        <div>
          <label className="block text-sm font-medium text-gray-700 mb-1">
            Freelancer Address
          </label>
          <input
            type="text"
            value={freelancer}
            onChange={(e) => setFreelancer(e.target.value)}
            className="w-full border border-gray-300 rounded-lg px-3 py-2 text-sm"
            placeholder="G... or C..."
            required
          />
        </div>
        <div>
          <label className="block text-sm font-medium text-gray-700 mb-1">
            Total Amount (USDC)
          </label>
          <input
            type="number"
            value={amount}
            onChange={(e) => setAmount(e.target.value)}
            className="w-full border border-gray-300 rounded-lg px-3 py-2 text-sm"
            placeholder="1000"
            min="1"
            required
          />
        </div>
      </div>

      <div>
        <label className="block text-sm font-medium text-gray-700 mb-1">
          Expiry Ledger
        </label>
        <input
          type="number"
          value={expiry}
          onChange={(e) => setExpiry(e.target.value)}
          className="w-full border border-gray-300 rounded-lg px-3 py-2 text-sm"
        />
      </div>

      <div>
        <div className="flex items-center justify-between mb-2">
          <label className="text-sm font-medium text-gray-700">
            Milestones
          </label>
          <button
            type="button"
            onClick={addMilestone}
            className="text-sm text-trust-600 flex items-center gap-1 hover:text-trust-700"
          >
            <Plus className="w-4 h-4" /> Add
          </button>
        </div>

        {!isValid && (
          <p className="text-xs text-red-600 mb-2">
            Percentages must sum to 100% (currently {totalPct / 100}%)
          </p>
        )}

        <div className="space-y-2">
          {milestones.map((m, i) => (
            <div key={i} className="flex items-center gap-3 bg-white border rounded-lg p-3">
              <span className="text-xs text-gray-500 w-20">Milestone {i + 1}</span>
              <input
                type="text"
                value={m.description}
                onChange={(e) => updateMilestone(i, "description", e.target.value)}
                className="flex-1 border border-gray-200 rounded px-2 py-1 text-sm"
                placeholder="Description"
              />
              <input
                type="number"
                value={m.percentage / 100}
                onChange={(e) =>
                  updateMilestone(i, "percentage", Number(e.target.value) * 100)
                }
                className="w-20 border border-gray-200 rounded px-2 py-1 text-sm"
                min="0"
                max="100"
              />
              <span className="text-xs text-gray-500 w-8">%</span>
              {milestones.length > 1 && (
                <button
                  type="button"
                  onClick={() => removeMilestone(i)}
                  className="text-red-500 hover:text-red-700"
                >
                  <Trash2 className="w-4 h-4" />
                </button>
              )}
            </div>
          ))}
        </div>
      </div>

      <button
        type="submit"
        disabled={!wallet?.isConnected || !isValid || submitting}
        className="w-full bg-trust-600 text-white py-2.5 rounded-lg font-medium hover:bg-trust-700 disabled:opacity-50 disabled:cursor-not-allowed transition"
      >
        {submitting ? "Creating..." : "Create Escrow"}
      </button>
    </form>
  )
}
