use rust_blockchain_implementation::utils::wallet::User;
use rust_blockchain_implementation::utils::transaction::Transaction;
use rust_blockchain_implementation::utils::block::Block;
use rust_blockchain_implementation::utils::blockchain::Blockchain;

// --- IMPORTS ---
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use rsa::pkcs8::EncodePublicKey;
use hex;

fn main() {
    println!("=== RUST BLOCKCHAIN SIMULATION ===\n");

    // 1. INITIALIZATION
    println!("Initializing 3 Users (Generating RSA Keys)...");
    let mut users = Vec::new();
    for i in 0..3 {
        let new_user = User::new(i);
        // We use 'new_user.id' here, so the compiler is happy!
        println!("  User {} wallet generated.", new_user.id); 
        users.push(new_user);
    }
    println!("Users initialized.\n");

    // Initialize Shared Ledger
    let ledger = Arc::new(Mutex::new(Blockchain::new()));

    // 2. PREPARE DATA FOR THREADS
    let ledger_ref1 = Arc::clone(&ledger);
    let user0_priv = users[0].private_key.clone();
    let user0_pub = users[0].public_key.clone();

    // --- SCENARIO 1: GENESIS BLOCK ---
    let handle1 = thread::spawn(move || {
        println!("--- Thread 1: Genesis Block ---");
        
        let user0 = User { id: 0, private_key: user0_priv, public_key: user0_pub.clone() };
        let user0_pub_bytes = user0_pub.to_public_key_der().unwrap().to_vec();

        let tx = Transaction::new(&user0, user0_pub_bytes, "Genesis".to_string());

        let mut locked_ledger = ledger_ref1.lock().unwrap();
        let prev_hash = locked_ledger.get_latest_hash();
        
        let block = Block::new(vec![tx], prev_hash);
        locked_ledger.add_block(block);
        println!("✅ Genesis Block added.\n");
    });
    handle1.join().unwrap();


    let ledger_ref2 = Arc::clone(&ledger);
    let user1_priv = users[1].private_key.clone();
    let user1_pub = users[1].public_key.clone();
    let user2_pub = users[2].public_key.clone();

    // --- SCENARIO 2: VALID TRANSACTION ---
    let handle2 = thread::spawn(move || {
        thread::sleep(Duration::from_secs(1)); 
        println!("--- Thread 2: Valid Transaction ---");
        
        let sender = User { id: 1, private_key: user1_priv, public_key: user1_pub };
        let receiver_pub_bytes = user2_pub.to_public_key_der().unwrap().to_vec();
        
        let tx = Transaction::new(&sender, receiver_pub_bytes, "Pay 50 BTC".to_string());

        if tx.verify_signature() {
            let mut locked_ledger = ledger_ref2.lock().unwrap();
            let prev_hash = locked_ledger.get_latest_hash();
            
            let block = Block::new(vec![tx], prev_hash);
            locked_ledger.add_block(block);
            println!("✅ Valid Transaction Block added.\n");
        }
    });
    handle2.join().unwrap();


    // --- SCENARIO 3: FORGERY ---
    let user2_priv = users[2].private_key.clone();
    let user2_pub = users[2].public_key.clone();
    let user0_pub_target = users[0].public_key.clone();

    let handle3 = thread::spawn(move || {
        thread::sleep(Duration::from_secs(1));
        println!("--- Thread 3: Forgery Attack ---");
        
        let attacker = User { id: 2, private_key: user2_priv, public_key: user2_pub };
        let mut forged_tx = Transaction::new(&attacker, vec![], "Steal 100 BTC".to_string());
        
        // Attack: Swap the sender key to impersonate User 0
        forged_tx.sender_pubkey = user0_pub_target.to_public_key_der().unwrap().to_vec();

        if forged_tx.verify_signature() {
            println!("❌ ERROR: Forged transaction accepted!");
        } else {
            println!("🛡️  SECURITY: Signature verification failed.\n");
        }
    });
    handle3.join().unwrap();


    // --- SCENARIO 4: TAMPERING ---
    let ledger_ref4 = Arc::clone(&ledger);
    let handle4 = thread::spawn(move || {
        thread::sleep(Duration::from_secs(1));
        println!("--- Thread 4: Tampering Attack ---");
        let mut locked_ledger = ledger_ref4.lock().unwrap();
        
        if let Some(block) = locked_ledger.chain.last_mut() {
            // Attack: Modify data in the ledger
            block.transactions[0].data = "HACKED DATA".to_string();
            
            if block.verify_integrity() {
                println!("❌ ERROR: Tampered block passed integrity check!");
            } else {
                println!("🛡️  SECURITY: Tampering detected (Hash Mismatch).\n");
            }
        }
    });
    handle4.join().unwrap();

    // --- FINAL PRINTING (This uses the receiver_pubkey, fixing the warning) ---
    let final_ledger = ledger.lock().unwrap();
    println!("=== FINAL LEDGER STATE ===");
    for (i, block) in final_ledger.chain.iter().enumerate() {
        let tx = &block.transactions[0];
        
        // Convert the receiver's key bytes to a hex string so we can read it
        let receiver_hex = hex::encode(&tx.receiver_pubkey);
        
        // We only print the first 10 characters of the key to keep the output clean
        let safe_display_key = if receiver_hex.len() > 10 { 
            &receiver_hex[0..10] 
        } else { 
            "None" 
        };

        println!("Block {}: [Tx: {}] [Receiver: {}...] [Hash: {}]", 
            i, 
            tx.data, 
            safe_display_key,
            block.hash
        );
    }
}