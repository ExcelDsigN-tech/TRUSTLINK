"use client"

import Link from "next/link"
import { WalletConnect } from "./WalletConnect"

export function Navbar() {
  return (
    <nav className="bg-white border-b border-gray-200 sticky top-0 z-50">
      <div className="max-w-7xl mx-auto px-4 h-16 flex items-center justify-between">
        <div className="flex items-center gap-8">
          <Link href="/" className="text-xl font-bold text-trust-600">
            TrustLink
          </Link>
          <div className="hidden md:flex items-center gap-6 text-sm font-medium">
            <Link
              href="/escrow"
              className="text-gray-600 hover:text-trust-600 transition"
            >
              Escrows
            </Link>
            <Link
              href="/disputes"
              className="text-gray-600 hover:text-trust-600 transition"
            >
              Disputes
            </Link>
            <Link
              href="/reputation"
              className="text-gray-600 hover:text-trust-600 transition"
            >
              Reputation
            </Link>
          </div>
        </div>
        <WalletConnect />
      </div>
    </nav>
  )
}
