# Architecture

## Actors

- **Sponsor**: creates a grant and controls milestone approval.
- **Builder**: submits evidence references and receives released funds.
- **Portaldot contract**: holds escrow, records state, emits evidence/release events, and enforces authorization.

## Flow

```text
Sponsor/Builder wallet
        │
        ▼
Frontend dashboard ── injected provider / RPC ──► PortaGrant ink! contract
                                                     │
                          ┌──────────────────────────┼──────────────────────────┐
                          ▼                          ▼                          ▼
                    Grant registry            Evidence records             Escrow release
```

## Invariants

1. `create_grant` accepts only matching non-empty title/amount arrays.
2. Every milestone amount is positive and the transferred value equals the checked total.
3. Only the builder can submit evidence for its grant.
4. Only the sponsor can approve a submitted milestone.
5. A milestone can be released once; the contract marks the grant `Completed` only after all releases.
6. No private key or seed phrase is handled by the frontend or committed to the repository.

## Extension path

- Replace evidence strings with content-addressed hashes and optional IPFS/Arweave links.
- Add a token-asset escrow adapter for ERC-20-compatible assets.
- Add an EVM-facing adapter for Portaldot V3's dual execution environment.
- Add indexer-backed grant discovery and community voting without changing escrow rules.
