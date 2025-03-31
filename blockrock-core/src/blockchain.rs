use std::collections::HashMap;
use ed25519_dalek::VerifyingKey;
use crate::transaction::Transaction;
use crate::block::Block;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Blockchain {
    pub blocks: Vec<Block>,
    pub authority: String,
    pub balances: HashMap<String, f64>,
    pub public_keys: HashMap<String, VerifyingKey>,
}

impl Blockchain {
    pub fn new(authority: String) -> Self {
        let mut blockchain = Blockchain {
            blocks: Vec::new(),
            authority,
            balances: HashMap::new(),
            public_keys: HashMap::new(),
        };
        
        blockchain.balances.insert("System".to_string(), 1000.0);
        blockchain.balances.insert("Alice".to_string(), 100.0);
        blockchain.balances.insert("Bob".to_string(), 50.0);
        blockchain.balances.insert("Charlie".to_string(), 30.0);

        let genesis_transaction = Transaction {
            sender: "System".to_string(),
            receiver: "Genesis".to_string(),
            amount: 0.0,
            signature: None,
        };
        
        let genesis_block = Block::new(0, vec![genesis_transaction], "0".to_string(), blockchain.authority.clone());
        blockchain.blocks.push(genesis_block);
        blockchain
    }

    pub fn add_public_key(&mut self, user: &str, public_key: VerifyingKey) {
        self.public_keys.insert(user.to_string(), public_key);
    }

    pub fn validate_transaction(&self, transaction: &Transaction) -> bool {
        if transaction.sender == "System" {
            return true;
        }

        match self.public_keys.get(&transaction.sender) {
            Some(public_key) => {
                if !transaction.verify(public_key) {
                    println!("Firma non valida per: {}", transaction.to_string());
                    return false;
                }
            }
            None => {
                println!("Chiave pubblica non trovata per: {}", transaction.sender);
                return false;
            }
        }

        match self.balances.get(&transaction.sender) {
            Some(balance) => *balance >= transaction.amount,
            None => {
                println!("Utente non trovato: {}", transaction.sender);
                false
            }
        }
    }

    pub fn update_balances(&mut self, transaction: &Transaction) {
        if transaction.sender != "System" {
            if let Some(sender_balance) = self.balances.get_mut(&transaction.sender) {
                *sender_balance -= transaction.amount;
            }
        }
        *self.balances.entry(transaction.receiver.clone()).or_insert(0.0) += transaction.amount;
    }

    pub fn add_block(&mut self, transactions: Vec<Transaction>, authority: String) -> bool {
        if authority != self.authority {
            println!("Autorità non valida: {}", authority);
            return false;
        }

        for tx in &transactions {
            if !self.validate_transaction(tx) {
                println!("Transazione non valida: {}", tx.to_string());
                return false;
            }
        }

        for tx in &transactions {
            self.update_balances(tx);
        }

        let prev_block = self.blocks.last().unwrap();
        let new_block = Block::new(
            prev_block.index + 1,
            transactions,
            prev_block.hash.clone(),
            authority,
        );
        
        self.blocks.push(new_block);
        true
    }
}
