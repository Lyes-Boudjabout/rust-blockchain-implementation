use chrono::Utc;
use sha2::{Sha256, Digest};
use super::transaction::Transaction;

const DIFFICULTY_PREFIX: &str = "0";

#[derive(Debug, Clone)]
pub struct Block {
    pub timestamp: i64,
    pub transactions: Vec<Transaction>,
    pub prev_hash: String,
    pub merkle_root: String,
    pub nonce: u64,
    pub hash: String,
}

fn calculate_hash(input: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(input);
    hex::encode(hasher.finalize())
}

fn calculate_merkle_root(transactions: &[Transaction]) -> String {
    if transactions.is_empty() { return String::new(); }
    let mut hashes: Vec<String> = transactions.iter().map(|tx| tx.hash.clone()).collect();

    while hashes.len() > 1 {
        let mut new_level = Vec::new();
        for chunk in hashes.chunks(2) {
            let left = chunk[0].clone();
            let right = if chunk.len() > 1 { chunk[1].clone() } else { chunk[0].clone() };
            new_level.push(calculate_hash(&format!("{}{}", left, right)));
        }
        hashes = new_level;
    }
    hashes[0].clone()
}

impl Block {
    pub fn new(transactions: Vec<Transaction>, prev_hash: String) -> Self {
        let timestamp = Utc::now().timestamp();
        let merkle_root = calculate_merkle_root(&transactions);
        
        let mut block = Block {
            timestamp,
            transactions,
            prev_hash,
            merkle_root,
            nonce: 0,
            hash: String::new(),
        };
        block.mine();
        block
    }

    pub fn mine(&mut self) {
        loop {
            let data = format!("{}{}{}{}", self.timestamp, self.nonce, self.merkle_root, self.prev_hash);
            let hash = calculate_hash(&data);
            
            if hash.starts_with(DIFFICULTY_PREFIX) {
                self.hash = hash;
                println!("⛏️  Block Mined! Nonce: {}, Hash: {}", self.nonce, self.hash);
                break;
            }
            self.nonce += 1;
        }
    }

    pub fn verify_integrity(&self) -> bool {
        // CHECK 1: Do the transactions match their own hashes?
        // This catches the "Tampering Attack" where someone changes data but not the hash.
        for tx in &self.transactions {
            if !tx.check_data_integrity() {
                return false;
            }
        }

        // CHECK 2: Does the Merkle Root match the transactions?
        let calc_merkle = calculate_merkle_root(&self.transactions);
        if calc_merkle != self.merkle_root { return false; }

        // CHECK 3: Is the Proof of Work valid?
        let data = format!("{}{}{}{}", self.timestamp, self.nonce, self.merkle_root, self.prev_hash);
        let calc_hash = calculate_hash(&data);
        
        calc_hash == self.hash && calc_hash.starts_with(DIFFICULTY_PREFIX)
    }
}