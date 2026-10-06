# Architecture

Wallet and sponsor/builder actions flow through the web dashboard into the Portaldot RPC and `PortaGrant` ink! contract. The contract stores grant metadata, milestone evidence references, and escrow state. Only the sponsor can approve a submitted milestone; only the builder can submit evidence.