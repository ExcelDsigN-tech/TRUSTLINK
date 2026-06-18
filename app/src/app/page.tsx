import Link from "next/link"

export default function Home() {
  return (
    <div className="text-center py-20">
      <h1 className="text-5xl font-bold text-trust-600 mb-4">TrustLink</h1>
      <p className="text-xl text-gray-600 mb-8 max-w-2xl mx-auto">
        Decentralized B2B escrow platform on Stellar Soroban.
        Secure milestone-based payments for cross-border trade.
      </p>
      <div className="flex items-center justify-center gap-4">
        <Link
          href="/escrow"
          className="bg-trust-600 text-white px-6 py-3 rounded-lg font-medium hover:bg-trust-700 transition"
        >
          Explore Escrows
        </Link>
        <Link
          href="/escrow/new"
          className="border border-trust-600 text-trust-600 px-6 py-3 rounded-lg font-medium hover:bg-trust-50 transition"
        >
          Create Escrow
        </Link>
      </div>
      <div className="grid grid-cols-1 md:grid-cols-3 gap-6 mt-16 max-w-4xl mx-auto">
        <div className="bg-white rounded-xl border border-gray-200 p-6 text-left">
          <h3 className="font-semibold mb-2">Milestone Escrow</h3>
          <p className="text-sm text-gray-500">
            Release funds in stages as work is completed and approved.
          </p>
        </div>
        <div className="bg-white rounded-xl border border-gray-200 p-6 text-left">
          <h3 className="font-semibold mb-2">Dispute Resolution</h3>
          <p className="text-sm text-gray-500">
            Fair on-chain arbitration with evidence-based rulings.
          </p>
        </div>
        <div className="bg-white rounded-xl border border-gray-200 p-6 text-left">
          <h3 className="font-semibold mb-2">Reputation Scores</h3>
          <p className="text-sm text-gray-500">
            Verifiable completion history for all participants.
          </p>
        </div>
      </div>
    </div>
  )
}
