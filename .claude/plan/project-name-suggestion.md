## Naming Recommendation: Project Rename

> **Current name:** TRUSTLINK (directory) / RepoFlow (source code)
> **Project type:** Decentralized B2B escrow & cross-border trade settlement on Stellar/Soroban

### Analysis

**Problems with "TRUSTLINK":**
- Generic — countless crypto projects use "trust" + "link" compounds
- No differentiation from hundreds of "Trust-X" brands
- Doesn't convey milestone-based escrow (the core differentiator)
- Sounds like a generic VPN/security product

**Problems with "RepoFlow":**
- Focused on OSS funding / repo claiming (old identity)
- Mismatched with B2B escrow use case

---

### Recommended Names (ranked)

#### 1. **EscrowFlow** ⭐ (Recommended)
| Aspect | Rating |
|--------|--------|
| Memorability | High — familiar compound, easy to spell |
| Domain fit | Unique, likely available as `.com` / `.io` |
| Value clarity | "Escrow" is instantly understood, "Flow" = seamless process |
| Brand continuity | Preserves "Flow" from existing codebase (repoflow → escrowflow), minimal rename pain |
| Ecosystem tie | Flows naturally with "Stellar" (stellar flow) |
| Trademark risk | Low — no major products use this exact compound |
| **Why it wins** | Clear value prop + minimal code rename effort + professional B2B tone |

#### 2. **SettleWave**
| Aspect | Rating |
|--------|--------|
| Memorability | High |
| Domain fit | Likely available |
| Value clarity | "Settle" = settlement, "Wave" = Stellar Wave ecosystem |
| Brand continuity | New identity (no ties to old code), clean break |
| Trademark risk | Low |
| **Why #2** | Strong ecosystem branding, good for Stellar-native marketing |

#### 3. **TrustBridge**
| Aspect | Rating |
|--------|--------|
| Memorability | High |
| Value clarity | "Bridge" = connecting B2B parties across borders |
| Issue | Still uses "Trust" prefix (slightly generic) |
| Best for | If you want to keep "Trust" in the name but improve distinctiveness |

#### 4. **DealShield**
| Aspect | Rating |
|--------|--------|
| Memorability | Medium |
| Value clarity | "Deal" = B2B agreements, "Shield" = protection (escrow) |
| Issue | Slightly consumer-focused tone |
| Best for | If targeting freelancers + small biz vs enterprise |

#### 5. **EscrowVault**
| Aspect | Rating |
|--------|--------|
| Memorability | Medium |
| Value clarity | "Vault" = safe storage of funds |
| Issue | Emphasizes storage over process flow |
| Best for | If security/audit is the primary marketing angle |

---

### Recommendation: EscrowFlow

**Rationale:**
1. **Codebase continuity** — Changing `repoflow-*` → `escrowflow-*` is a mechanical rename with no architectural change. All package names, env vars, DB names, and URLs follow the `*flow` pattern.
2. **Value clarity** — "Escrow" is the highest-signal keyword for your target audience (B2B trade partners). No guessing required.
3. **Memorability** — Two common English words, easy to spell, works internationally.
4. **Positioning** — Positions the product as a *process* platform (flow of escrow) rather than a static vault.

**Evolution path:**
```
RepoFlow (OSS funding) → TrustLink (placeholder) → EscrowFlow (B2B escrow)
```

**Tagline ideas:**
- *"Milestone-based escrow on Stellar"*
- *"Trustless trade settlement"*
- *"B2B payments, secured by Soroban"*
