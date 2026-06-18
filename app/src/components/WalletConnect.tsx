"use client"

import { useCallback, useEffect, useState } from "react"
import { connectWallet } from "@/lib/freighter"
import type { WalletState } from "@/types"

export function WalletConnect() {
  const [wallet, setWallet] = useState<WalletState>({
    address: null,
    isConnected: false,
    isFreighter: false,
  })

  useEffect(() => {
    const saved = localStorage.getItem("trustlink_wallet")
    if (saved) {
      setWallet(JSON.parse(saved))
    }
  }, [])

  const handleConnect = useCallback(async () => {
    const state = await connectWallet()
    if (state.isConnected) {
      localStorage.setItem("trustlink_wallet", JSON.stringify(state))
    }
    setWallet(state)
  }, [])

  const handleDisconnect = useCallback(() => {
    localStorage.removeItem("trustlink_wallet")
    setWallet({ address: null, isConnected: false, isFreighter: false })
  }, [])

  if (wallet.isConnected && wallet.address) {
    return (
      <div className="flex items-center gap-3">
        <span className="text-xs text-gray-500 font-mono">
          {wallet.address.slice(0, 6)}...{wallet.address.slice(-4)}
        </span>
        <button
          onClick={handleDisconnect}
          className="text-xs text-red-600 hover:text-red-800 transition"
        >
          Disconnect
        </button>
      </div>
    )
  }

  return (
    <button
      onClick={handleConnect}
      className="bg-trust-600 text-white px-4 py-2 rounded-lg text-sm font-medium hover:bg-trust-700 transition"
    >
      {wallet.isFreighter ? "Connect Wallet" : "Install Freighter"}
    </button>
  )
}
