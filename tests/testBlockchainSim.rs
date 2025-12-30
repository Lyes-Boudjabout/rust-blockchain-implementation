// tests/simulation_tests.rs

use rust_blockchain_simulation::utils::wallet::User;
use rust_blockchain_simulation::utils::transaction::Transaction;
use rust_blockchain_simulation::utils::block::Block;
use rust_blockchain_simulation::utils::blockchain::Blockchain;
use rsa::pkcs8::EncodePublicKey;

// --- TEST 1: TRANSACTION SIGNATURES ---
#[test]
fn test_transaction_signing_and_verification() {
    let sender = User::new(0);
    let receiver = User::new(1);
    let receiver_pub_bytes = receiver.public_key.to_public_key_der().unwrap().to_vec();

    let tx = Transaction::new(&sender, receiver_pub_bytes, "Valid Data".to_string());
    
    assert!(tx.verify_signature(), "Valid signature should return true");
    assert!(tx.check_data_integrity(), "Data integrity check should pass");
}

#[test]
fn test_forged_transaction_detection() {
    let sender = User::new(0);
    let attacker = User::new(2); 
    let receiver = User::new(1);
    let receiver_pub_bytes = receiver.public_key.to_public_key_der().unwrap().to_vec();
    
    // Attacker signs with THEIR key
    let mut forged_tx = Transaction::new(&attacker, receiver_pub_bytes, "Steal Money".to_string());
    
    // But claims it is from SENDER
    forged_tx.sender_pubkey = sender.public_key.to_public_key_der().unwrap().to_vec();

    assert!(!forged_tx.verify_signature(), "Forged signature should verify false");
}

// --- TEST 2: BLOCK MINING ---
#[test]
fn test_block_mining_difficulty() {
    let user = User::new(0);
    let receiver_pub = user.public_key.to_public_key_der().unwrap().to_vec();
    let tx = Transaction::new(&user, receiver_pub, "Test Mining".to_string());
    
    let prev_hash = String::from("0000abc123");
    let block = Block::new(vec![tx], prev_hash);

    assert!(block.hash.starts_with("0"), "Mined block hash must meet difficulty criteria");
}

// --- TEST 3: BLOCKCHAIN LINKING ---
#[test]
fn test_blockchain_linking() {
    let mut blockchain = Blockchain::new();
    let user = User::new(0);
    let receiver_pub = user.public_key.to_public_key_der().unwrap().to_vec();

    // Block 1
    let tx1 = Transaction::new(&user, receiver_pub.clone(), "Block 1".to_string());
    let prev_hash1 = blockchain.get_latest_hash();
    let block1 = Block::new(vec![tx1], prev_hash1);
    blockchain.add_block(block1.clone());

    // Block 2
    let tx2 = Transaction::new(&user, receiver_pub, "Block 2".to_string());
    let prev_hash2 = blockchain.get_latest_hash();
    let block2 = Block::new(vec![tx2], prev_hash2);
    blockchain.add_block(block2.clone());

    assert_eq!(blockchain.chain[1].prev_hash, block1.hash, "Block 2 must link to Block 1");
}

// --- TEST 4: TAMPERING ---
#[test]
fn test_tampering_detection() {
    let user = User::new(0);
    let receiver_pub = user.public_key.to_public_key_der().unwrap().to_vec();
    let tx = Transaction::new(&user, receiver_pub, "Original Data".to_string());
    
    let prev_hash = String::from("0000valid");
    let mut block = Block::new(vec![tx], prev_hash);

    assert!(block.verify_integrity(), "Initial block should be valid");

    // Attack: Modify the data in memory
    block.transactions[0].data = "Hacked Data".to_string();

    assert!(!block.verify_integrity(), "Tampered block should fail integrity check");
}