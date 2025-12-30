# Rust Blockchain Simulation

A functional, multi-threaded blockchain simulation written in **Rust**. This project demonstrates the core cryptographic and consensus mechanisms that power cryptocurrencies like Bitcoin, including Proof of Work, Digital Signatures (RSA), Merkle Trees, and immutable ledger linking.

## 🚀 Features

* **Proof of Work (Mining):** Implements a difficulty-based consensus algorithm (simulated with leading zeros).
* **Digital Signatures:** Uses **RSA-2048** and **SHA-256** to sign and verify every transaction, ensuring identity theft protection.
* **Immutable Ledger:** Blocks are cryptographically linked; altering an old block invalidates the entire chain.
* **Merkle Trees:** Efficiently verifies the integrity of transactions within a block.
* **Concurrency:** Uses Rust's `Arc` and `Mutex` to simulate a decentralized network with multiple threads acting as independent nodes.
* **Attack Simulations:** Includes built-in scenarios that actively attempt to forge signatures and tamper with ledger data to demonstrate security defenses.

## 📂 Project Structure

The logic is modularized for clarity and separation of concerns:

| File | Description |
| :--- | :--- |
| `src/main.rs` | **The Orchestrator.** Runs the multi-threaded simulation and attack scenarios. |
| `src/wallet.rs` | **Identity.** Manages User structs and generates RSA Public/Private key pairs. |
| `src/transaction.rs` | **The Data.** Handles transaction creation, SHA-256 hashing, and RSA signing/verification. |
| `src/block.rs` | **The Container.** Handles Merkle Root calculation and the Mining loop (Proof of Work). |
| `src/blockchain.rs` | **The Ledger.** Manages the chain vector and block linking. |

## 🛠️ Prerequisites

* **Rust & Cargo:** You need the Rust toolchain installed.
    * Install via: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`

## 📦 Installation & Usage

1.  **Clone the repository:**
    ```bash
    git clone [https://github.com/your-username/rust-blockchain-sim.git](https://github.com/your-username/rust-blockchain-sim.git)
    cd rust-blockchain-sim
    ```

2.  **Build the project:**
    ```bash
    cargo build
    ```

3.  **Run the simulation:**
    ```bash
    cargo run
    ```

## 🖥️ Simulation Scenarios

When you run the program, it executes four distinct scenarios to test the system:

### 1. Genesis Block
**Action:** User 0 creates the first transaction to bootstrap the network.
**Result:** ✅ The block is mined and added to the ledger.

### 2. Valid Transfer
**Action:** User 1 signs a transaction sending "50 BTC" to User 2.
**Result:** ✅ The signature is verified, the block is mined, and the chain grows.

### 3. Forgery Attack (The Identity Theft)
**Action:** User 2 (Attacker) creates a transaction claiming to be User 0 but signs it with their own key.
**Result:** 🛡️ **Blocked.** The system detects the signature mismatch and rejects the transaction before mining begins.

### 4. Tampering Attack (The History Rewrite)
**Action:** An attacker modifies data ("Pay 50 BTC" -> "HACKED DATA") inside an already mined block in memory.
**Result:** ❌ **Detected.** The integrity check fails because the modified data does not match the stored hash/Merkle root.

## 📝 Example Output

```text
=== RUST BLOCKCHAIN SIMULATION ===

Initializing 3 Users (Generating RSA Keys)...
  User 0 wallet generated.
  User 1 wallet generated.
  User 2 wallet generated.
Users initialized.

--- Thread 1: Genesis Block ---
⛏️  Block Mined! Nonce: 8, Hash: 0c55f...
✅ Genesis Block added.

--- Thread 2: Valid Transaction ---
⛏️  Block Mined! Nonce: 7, Hash: 0908b...
✅ Valid Transaction Block added.

--- Thread 3: Forgery Attack ---
🛡️  SECURITY: Signature verification failed.

--- Thread 4: Tampering Attack ---
🛡️  SECURITY: Tampering detected (Hash Mismatch).

=== FINAL LEDGER STATE ===
Block 0: [Tx: Genesis] [Receiver: 30820...] [Hash: 0c55f...]
Block 1: [Tx: Pay 50 BTC] [Receiver: 30820...] [Hash: 0908b...]
```

## 📚 Technical Stack

- Language: Rust 2021 Edition
- Hashing: sha2 (SHA-256)
- Encryption: rsa (PKCS#1 v1.5 signatures)
- Serialization: hex (Binary to Hex string conversion)
- Time: chrono (UTC timestamps)

## 📄 License
This project is open-source and available under the MIT License.