# Security notes

- Never commit `scripts/.env`, a private key, or a seed phrase.
- Use a dedicated testnet signer with only the minimum required balance.
- Verify the contract address and transaction hashes in the explorer before publishing evidence.
- Treat evidence as a public content reference; do not put personal or confidential data on-chain.
- Deployment scripts fail when required configuration is missing.
