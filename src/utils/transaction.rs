use sha2::{Sha256, Digest};
use rsa::{Pkcs1v15Sign, RsaPublicKey};
use rsa::pkcs8::{EncodePublicKey, DecodePublicKey};
use super::wallet::User;

#[derive(Debug, Clone)]
pub struct Transaction {
    pub sender_pubkey: Vec<u8>,
    pub receiver_pubkey: Vec<u8>,
    pub data: String,
    pub signature: Vec<u8>,
    pub hash: String,
}

impl Transaction {
    // 1. REUSABLE HELPER: Centralize how we calculate the hash
    fn compute_tx_hash(sender: &[u8], receiver: &[u8], data: &str) -> (Vec<u8>, String) {
        let content = format!("{}{}{}", 
            hex::encode(sender), 
            hex::encode(receiver), 
            data
        );

        let mut hasher = Sha256::new();
        hasher.update(&content);
        let hash_raw = hasher.finalize().to_vec(); // 32 bytes (for signing)
        let hash_hex = hex::encode(&hash_raw);     // Hex string (for storage)
        
        (hash_raw, hash_hex)
    }

    pub fn new(sender: &User, receiver_pubkey: Vec<u8>, data: String) -> Self {
        let sender_pub_der = sender.public_key.to_public_key_der().unwrap().to_vec();
        
        // Use the helper to get the hash
        let (hash_raw, hash_hex) = Self::compute_tx_hash(&sender_pub_der, &receiver_pubkey, &data);

        // Sign the raw hash
        let padding = Pkcs1v15Sign::new::<Sha256>();
        let signature = sender.private_key
            .sign(padding, &hash_raw)
            .expect("Signing failed");

        Transaction {
            sender_pubkey: sender_pub_der,
            receiver_pubkey,
            data,
            signature,
            hash: hash_hex,
        }
    }

    pub fn verify_signature(&self) -> bool {
        let pub_key = RsaPublicKey::from_public_key_der(&self.sender_pubkey).unwrap();
        let padding = Pkcs1v15Sign::new::<Sha256>();
        let hash_raw = hex::decode(&self.hash).unwrap_or_default();
        pub_key.verify(padding, &hash_raw, &self.signature).is_ok()
    }

    // 2. NEW INTEGRITY CHECK: Re-calculate hash from data and compare with stored hash
    pub fn check_data_integrity(&self) -> bool {
        let (_, calculated_hex) = Self::compute_tx_hash(
            &self.sender_pubkey, 
            &self.receiver_pubkey, 
            &self.data
        );
        calculated_hex == self.hash
    }
}