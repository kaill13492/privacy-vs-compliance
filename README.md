# privacy-vs-compliance

# Private Transfer Starter – Halo2

A simple, educational starter project demonstrating a basic **shielded-like private transfer** using **Halo2** (zk-SNARK proofs).

This circuit proves knowledge of a valid transfer:
- Hides the sender's old balance and transfer amount
- Uses Poseidon hash commitments for both sender and receiver
- Keeps everything private except the public commitments

Goal: Provide an easy entry point for experimenting with privacy-preserving payments on Ethereum-like chains, with future potential for **selective disclosure** (e.g., reveal amount to auditors/regulators via viewing keys).

## Current Status
Minimal working version (commit-and-verify style shielded transfer).

- Private inputs: sender old balance, amount, two blinding factors
- Public outputs: sender & receiver commitments
- Verified via MockProver

## Roadmap / To-Do
- [ ] Add Merkle proof for note existence (shielded pool membership)
- [ ] Implement viewing key + selective disclosure logic
- [ ] Add nullifier to prevent double-spending
- [ ] Integration tests with ethers-rs / Foundry
- [ ] Compare gas/performance with Plonk, Groth16, or Noir
- [ ] Extend to full shielded note model (inspired by Zcash Orchard / Aztec)

## Requirements
- Rust 1.75 or newer
- Cargo

## Quick Start

```bash
# 1. Clone the repo
git clone https://github.com/YOUR-USERNAME/private-transfer-starter.git
cd private-transfer-starter

# 2. Build & run the example
cargo run

Simple private transfer circuit works! (mock proof passed)
Next: add Merkle proof for note existence, viewing keys for selective reveal.

cargo test

private-transfer-starter/
├── Cargo.toml
├── src/
│   └── main.rs          # Main circuit definition + mock prover example
└── README.md

Dependencies (as of January 2026)

halo2_proofs       → from privacy-scaling-explorations/halo2 (main branch)
halo2wrong         → extra gadgets (Poseidon, etc.)
rand_core

Recommended Resources

Official Halo2 Book – core concepts & API
Halo2wrong repo – useful gadgets
Aztec Noir – higher-level privacy language (easier for contracts)
Semaphore Protocol – great for anonymous signaling / selective reveal

License
MIT
Contributions welcome!
Fork → branch → PR with viewing keys, nullifiers, or Merkle integration would be awesome. 🚀
Built for learning & tinkering with privacy vs compliance in blockchain.

ok
