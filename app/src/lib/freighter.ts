import type { WalletState } from "@/types"

let freighter: typeof import("@stellar/freighter-api") | null = null

async function getFreighter() {
  if (!freighter) {
    try {
      freighter = await import("@stellar/freighter-api")
    } catch {
      return null
    }
  }
  return freighter
}

export async function connectWallet(): Promise<WalletState> {
  const f = await getFreighter()
  if (!f) return { address: null, isConnected: false, isFreighter: false }

  try {
    const { isConnected: connected } = await f.isConnected()
    if (!connected) return { address: null, isConnected: false, isFreighter: true }

    const { address } = await f.getAddress()
    if (!address) return { address: null, isConnected: false, isFreighter: true }

    return { address, isConnected: true, isFreighter: true }
  } catch {
    return { address: null, isConnected: false, isFreighter: true }
  }
}

export async function getNetworkPassphrase(): Promise<string | null> {
  const f = await getFreighter()
  if (!f) return null

  try {
    const { networkPassphrase } = await f.getNetwork()
    return networkPassphrase || null
  } catch {
    return null
  }
}

export async function signTransaction(
  xdr: string,
  networkPassphrase: string
): Promise<string | null> {
  const f = await getFreighter()
  if (!f) return null

  try {
    const { signedTxXdr } = await f.signTransaction(xdr, {
      networkPassphrase,
    })
    return signedTxXdr || null
  } catch {
    return null
  }
}
