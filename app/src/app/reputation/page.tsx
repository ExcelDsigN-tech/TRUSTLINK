"use client"

import { useState } from "react"
import { Search } from "lucide-react"
import { getReputation, getCompletionHistory } from "@/lib/api"
import type { Reputation, CompletionRecord } from "@/types"

export default function ReputationPage() {
  const [address, setAddress] = useState("")
  const [rep, setRep] = useState<Reputation | null>(null)
  const [history, setHistory] = useState<CompletionRecord[]>([])
  const [searched, setSearched] = useState(false)
  const [loading, setLoading] = useState(false)

  const handleSearch = async (e: React.FormEvent) => {
    e.preventDefault()
    if (!address.trim()) return

    setLoading(true)
    setSearched(true)
    try {
      const [repData, historyData] = await Promise.all([
        getReputation(address.trim()),
        getCompletionHistory(address.trim()),
      ])
      setRep(repData)
      setHistory(historyData)
    } finally {
      setLoading(false)
    }
  }

  return (
    <div className="max-w-3xl mx-auto">
      <h1 className="text-2xl font-bold mb-6">Reputation Explorer</h1>

      <form onSubmit={handleSearch} className="mb-8">
        <div className="flex gap-3">
          <input
            type="text"
            value={address}
            onChange={(e) => setAddress(e.target.value)}
            placeholder="Enter Stellar address (G... or C...)"
            className="flex-1 border border-gray-300 rounded-lg px-4 py-2.5 text-sm"
          />
          <button
            type="submit"
            disabled={loading}
            className="bg-trust-600 text-white px-6 py-2.5 rounded-lg font-medium hover:bg-trust-700 disabled:opacity-50 transition flex items-center gap-2"
          >
            <Search className="w-4 h-4" /> Search
          </button>
        </div>
      </form>

      {loading && (
        <div className="text-center py-12 text-gray-500">Searching...</div>
      )}

      {searched && !loading && !rep && (
        <div className="bg-white rounded-xl border border-gray-200 p-12 text-center">
          <p className="text-gray-500">
            No reputation data found for this address.
          </p>
        </div>
      )}

      {rep && (
        <>
          <div className="bg-white rounded-xl border border-gray-200 p-6 mb-6">
            <h2 className="text-lg font-semibold mb-4">Overview</h2>
            <div className="grid grid-cols-2 md:grid-cols-4 gap-4">
              <div className="text-center p-4 bg-gray-50 rounded-lg">
                <div className="text-2xl font-bold text-trust-600">
                  {rep.total_deals}
                </div>
                <div className="text-xs text-gray-500 mt-1">Total Deals</div>
              </div>
              <div className="text-center p-4 bg-green-50 rounded-lg">
                <div className="text-2xl font-bold text-green-600">
                  {rep.completed_deals}
                </div>
                <div className="text-xs text-gray-500 mt-1">Completed</div>
              </div>
              <div className="text-center p-4 bg-red-50 rounded-lg">
                <div className="text-2xl font-bold text-red-600">
                  {rep.disputed_deals}
                </div>
                <div className="text-xs text-gray-500 mt-1">Disputes</div>
              </div>
              <div className="text-center p-4 bg-blue-50 rounded-lg">
                <div className="text-2xl font-bold text-blue-600">
                  {rep.total_volume.toLocaleString()}
                </div>
                <div className="text-xs text-gray-500 mt-1">Volume</div>
              </div>
            </div>
          </div>

          {history.length > 0 && (
            <div className="bg-white rounded-xl border border-gray-200 p-6">
              <h2 className="text-lg font-semibold mb-4">
                Completion History
              </h2>
              <div className="space-y-2">
                {history.map((h) => (
                  <div
                    key={h.id}
                    className="flex items-center justify-between py-2 border-b border-gray-100 last:border-0"
                  >
                    <div>
                      <span className="text-sm font-medium">
                        Escrow #{h.escrow_id}
                      </span>
                      <span className="text-xs text-gray-500 ml-2">
                        with {h.counterparty.slice(0, 8)}...
                      </span>
                    </div>
                    <div className="text-right">
                      <span className="text-sm">
                        {h.amount.toLocaleString()} USDC
                      </span>
                      {h.had_dispute && (
                        <span className="text-xs text-red-500 ml-2">
                          Disputed
                        </span>
                      )}
                    </div>
                  </div>
                ))}
              </div>
            </div>
          )}
        </>
      )}
    </div>
  )
}
