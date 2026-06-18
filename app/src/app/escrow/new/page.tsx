"use client"

import { useRouter } from "next/navigation"
import { CreateEscrowForm } from "@/components/CreateEscrowForm"
import { useEffect, useState } from "react"
import type { WalletState } from "@/types"

export default function NewEscrowPage() {
  const router = useRouter()
  const [wallet, setWallet] = useState<WalletState | null>(null)

  useEffect(() => {
    const saved = localStorage.getItem("trustlink_wallet")
    if (saved) setWallet(JSON.parse(saved))
  }, [])

  return (
    <div className="max-w-2xl mx-auto">
      <h1 className="text-2xl font-bold mb-6">Create Escrow</h1>

      {!wallet?.isConnected ? (
        <div className="bg-yellow-50 border border-yellow-200 rounded-xl p-6 text-center">
          <p className="text-yellow-800 font-medium">
            Connect your Freighter wallet to create an escrow.
          </p>
        </div>
      ) : (
        <CreateEscrowForm
          wallet={wallet}
          onSuccess={() => router.push("/escrow")}
        />
      )}
    </div>
  )
}
