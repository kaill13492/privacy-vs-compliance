# Privacy vs Compliance – zk Transfers with Halo2

This repository explores the trade-off between **on-chain privacy** and **regulatory compliance** using
**zero-knowledge proofs (Halo2 / zk-SNARKs)**.

The goal is to demonstrate how **private value transfers** can coexist with
**auditing, selective disclosure, and regulatory requirements**.

---

## 🧠 Problem Statement

Blockchain systems face a fundamental tension:

- **Privacy** → users want hidden balances and transactions  
- **Compliance** → regulators require auditability, AML, and selective access  

This project shows how **zk-proofs** allow *both*.

---

## ✨ Key Features

- ✅ Hidden transfer amounts (zk-SNARKs)
- ✅ Commitment-based balances
- 🚧 Merkle tree for state inclusion
- 🚧 Selective disclosure (viewing keys)
- 🚧 Auditor / regulator access without breaking global privacy

---

## 🏗️ Architecture Overview

User
├─ creates commitment (value hidden)
├─ generates zk-proof (valid transfer)
└─ optionally shares viewing key
↓
Blockchain / Verifier
├─ verifies proof
└─ stores commitment hash
↓
Auditor (optional)
└─ verifies selective disclosure


---

## 🔐 Privacy vs Compliance

| Feature | Privacy-Only | This Model |
|-------|-------------|------------|
| Hidden amounts | ✅ | ✅ |
| Public audit | ❌ | ❌ |
| Selective disclosure | ❌ | ✅ |
| AML / audit ready | ❌ | ✅ |
| GDPR friendly | ✅ | ✅ |

---

## 🧪 Example Use Case

- User makes a private transfer
- Amount is hidden on-chain
- Regulator receives a **viewing key**
- Auditor verifies correctness **without revealing other transactions**

---

## 📂 Project Structure


---

## 🚀 Getting Started

```bash
cargo build
cargo test
📚 Resources

Halo2 Book

Zero-Knowledge Proofs (zkSNARKs)

Privacy-Preserving Compliance Research

⚠️ Disclaimer

This repository is educational / experimental
Not production-ready. No legal guarantees.

---

## 2️⃣ GitHub Actions CI  
`.github/workflows/ci.yml`

```yml
name: Rust CI

on:
  push:
  pull_request:

jobs:
  build:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      - name: Install Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
          override: true

      - name: Build
        run: cargo build --verbose

      - name: Test
        run: cargo test --verbose

use privacy_vs_compliance::disclosure::ViewingKey;

#[test]
fn selective_disclosure_works() {
    let vk = ViewingKey {
        tx_commitment: [1u8; 32],
        secret: [2u8; 32],
    };

    let proof = vk.disclose(100);

    assert_eq!(proof.revealed_amount, 100);
    assert_eq!(proof.commitment, [1u8; 32]);
}
# Selective Disclosure

Selective disclosure allows a user to reveal
**only specific transaction data** to a trusted auditor.

This avoids:
- full transaction history leaks
- public deanonymization
- non-compliance with GDPR

The mechanism relies on:
- commitments
- viewing keys
- zk-proof verification
