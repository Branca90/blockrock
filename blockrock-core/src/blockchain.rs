use std::collections::HashMap;
pub use crate::block::Block;
pub use crate::transaction::Transaction;
use ed25519_dalek::VerifyingKey;

#[derive(Debug)]
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
        
        // Inizializza saldi
        blockchain.balances.insert("System".to_string(), 1000.0);
        blockchain.balances.insert("Alice".to_string(), 100.0);
        blockchain.balances.insert("Bob".to_string(), 50.0);
        blockchain.balances.insert("Charlie".to_string(), 30.0);
        blockchain.balances.insert("Node1".to_string(), 50.0);
        blockchain.balances.insert("Node2".to_string(), 0.0);

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

    pub fn add_block(&mut self, transactions: Vec<Transaction>, authority: String) -> bool {
        let previous_hash = self.blocks.last()
            .map(|block| block.hash.clone())
            .unwrap_or("0".to_string());
        
        if !self.validate_transactions(&transactions) {
            return false;
        }

        let new_block = Block::new(
            self.blocks.len() as u32,
            transactions.clone(),
            previous_hash,
            authority,
        );

        for transaction in transactions {
            if transaction.sender != "System" {
                if let Some(sender_balance) = self.balances.get_mut(&transaction.sender) {
                    *sender_balance -= transaction.amount;
                }
            }
            if let Some(receiver_balance) = self.balances.get_mut(&transaction.receiver) {
                *receiver_balance += transaction.amount;
            } else {
                self.balances.insert(transaction.receiver.clone(), transaction.amount);
            }
        }

        self.blocks.push(new_block);
        true
    }

    fn validate_transactions(&self, transactions: &[Transaction]) -> bool {
        for transaction in transactions {
            if !self.validate_transaction(transaction) {
                println!("Transazione non valida: {} -> {}: {}", transaction.sender, transaction.receiver, transaction.amount);
                return false;
            }
        }
        true
    }

    fn validate_transaction(&self, transaction: &Transaction) -> bool {
        if transaction.sender == "System" {
            return true;
        }

        if let Some(sender_balance) = self.balances.get(&transaction.sender) {
            if *sender_balance >= transaction.amount {
                return true;
            } else {
                println!("Saldo insufficiente per {}: richiesto {}, disponibile {}", transaction.sender, transaction.amount, sender_balance);
                return false;
            }
        } else {
            println!("Utente non trovato: {}", transaction.sender);
            return false;
        }
    }

    pub fn get_blocks(&self) -> &Vec<Block> {
        &self.blocks
    }

    pub fn get_balances(&self) -> Vec<(String, f64)> {
        let mut balances: Vec<(String, f64)> = self.balances.iter()
            .map(|(k, v)| (k.clone(), *v))
            .collect();
        balances.sort_by(|a, b| a.0.cmp(&b.0));
        balances
    }

    pub fn add_public_key(&mut self, name: &str, key: VerifyingKey) {
        self.public_keys.insert(name.to_string(), key);
    }
}
