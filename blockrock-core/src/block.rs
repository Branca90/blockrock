use sha2::{Digest, Sha256};
use std::time::{SystemTime, UNIX_EPOCH};
use crate::transaction::Transaction;
use serde::{Serialize, Deserialize};
use hex;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Block {
    pub index: u32,
    pub timestamp: u64,
    pub transactions: Vec<Transaction>,
    pub previous_hash: String,
    pub hash: String,
    pub authority: String,
}

impl Block {
    pub fn new(index: u32, transactions: Vec<Transaction>, previous_hash: String, authority: String) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let hash = Block::calculate_hash(index, timestamp, &transactions, &previous_hash, &authority);
        Block {
            index,
            timestamp,
            transactions,
            previous_hash,
            hash,
            authority,
        }
    }

    fn calculate_hash(index: u32, timestamp: u64, transactions: &[Transaction], previous_hash: &str, authority: &str) -> String {
        let transactions_str: String = transactions.iter()
            .map(|tx| tx.to_string())
            .collect::<Vec<String>>()
            .join("|");
        
        let input = format!("{}{}{}{}{}", index, timestamp, transactions_str, previous_hash, authority);
        let mut hasher = Sha256::new();
        hasher.update(input);
        hex::encode(hasher.finalize())
    }
}
