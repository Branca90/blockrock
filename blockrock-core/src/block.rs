use serde::{Serialize, Deserialize};
use sha2::{Sha256, Digest};

use crate::transaction::Transaction;

#[derive(Debug, Clone, Serialize, Deserialize)]
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
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let mut block = Block {
            index,
            timestamp,
            transactions,
            previous_hash,
            hash: String::new(),
            authority,
        };
        block.hash = block.calculate_hash();
        block
    }

    pub fn calculate_hash(&self) -> String {
        let mut hasher = Sha256::new();
        hasher.update(self.index.to_le_bytes());
        hasher.update(self.timestamp.to_le_bytes());
        for tx in &self.transactions {
            hasher.update(tx.sender.as_bytes());
            hasher.update(tx.receiver.as_bytes());
            hasher.update(tx.amount.to_le_bytes());
            if let Some(signature) = &tx.signature {
                hasher.update(signature.to_bytes());
            }
        }
        hasher.update(self.previous_hash.as_bytes());
        hasher.update(self.authority.as_bytes());
        hex::encode(hasher.finalize())
    }
}
