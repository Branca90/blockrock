use sha2::{Digest, Sha256};
use std::time::{SystemTime, UNIX_EPOCH};
use std::collections::HashMap;
use ed25519_dalek::{Signer, Signature, SigningKey, VerifyingKey};
use rand::rngs::OsRng;
use ed25519_dalek::Verifier;


#[derive(Debug)]
struct Transaction {
    sender: String,
    receiver: String,
    amount: f64,
    signature: Option<Signature>,
}

impl Transaction {
    fn new(sender: String, receiver: String, amount: f64, signing_key: &SigningKey) -> Self {
        let mut tx = Self {
            sender,
            receiver,
            amount,
            signature: None,
        };
        tx.sign(signing_key);
        tx
    }

    fn to_string(&self) -> String {
        format!("{} -> {}: {}", self.sender, self.receiver, self.amount)
    }

    fn sign(&mut self, signing_key: &SigningKey) {
        let message_str = self.to_string();
        let message = message_str.as_bytes();
        self.signature = Some(signing_key.sign(message));
    }

    fn verify(&self, public_key: &VerifyingKey) -> bool {
        let message_str = self.to_string();
        let message = message_str.as_bytes();
        self.signature
            .as_ref()
            .map_or(false, |sig| public_key.verify(message, sig).is_ok())
    }
}

#[derive(Debug)]
struct Block {
    index: u32,
    timestamp: u64,
    transactions: Vec<Transaction>,
    previous_hash: String,
    hash: String,
    authority: String,
}

impl Block {
    fn new(index: u32, transactions: Vec<Transaction>, previous_hash: String, authority: String) -> Self {
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

struct Blockchain {
    blocks: Vec<Block>,
    authority: String,
    balances: HashMap<String, f64>,
    public_keys: HashMap<String, VerifyingKey>,
}

impl Blockchain {
    fn new(authority: String) -> Self {
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
        
        let genesis_block = Block::new(
            0,
            vec![genesis_transaction],
            "0".to_string(),
            blockchain.authority.clone()
        );
        
        blockchain.blocks.push(genesis_block);
        blockchain
    }

    fn add_public_key(&mut self, user: &str, public_key: VerifyingKey) {
        self.public_keys.insert(user.to_string(), public_key);
    }

    fn validate_transaction(&self, transaction: &Transaction) -> bool {
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

    fn update_balances(&mut self, transaction: &Transaction) {
        if transaction.sender != "System" {
            if let Some(sender_balance) = self.balances.get_mut(&transaction.sender) {
                *sender_balance -= transaction.amount;
            }
        }
        *self.balances.entry(transaction.receiver.clone()).or_insert(0.0) += transaction.amount;
    }

    fn add_block(&mut self, transactions: Vec<Transaction>, authority: String) -> bool {
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

    fn print_block(&self, block: &Block) {
        println!(
            "Blocco #{} [{}]\nHash: {}\nPrevious Hash: {}\nAuthority: {}\nTransazioni:",
            block.index, block.timestamp, block.hash, block.previous_hash, block.authority
        );
        
        for tx in &block.transactions {
            println!("- {}", tx.to_string());
        }
        println!();
    }

    fn print_balances(&self) {
        println!("Saldi attuali:");
        for (user, balance) in &self.balances {
            println!("- {}: {:.2}", user, balance);
        }
        println!();
    }
}

fn main() {
    println!("Avvio di BlockRock...\n");
    
    let authority = "Node1".to_string();
    let mut blockchain = Blockchain::new(authority.clone());

    let mut csprng = OsRng;
    
    // Generazione chiavi
    let alice_key = SigningKey::generate(&mut csprng);
    let bob_key = SigningKey::generate(&mut csprng);
    let charlie_key = SigningKey::generate(&mut csprng);

    // Registrazione chiavi pubbliche
    println!("Chiave pubblica di Alice: {:?}", alice_key.verifying_key());
    println!("Chiave pubblica di Bob: {:?}", bob_key.verifying_key());
    println!("Chiave pubblica di Charlie: {:?}", charlie_key.verifying_key());
    println!();

    blockchain.add_public_key("Alice", alice_key.verifying_key());
    blockchain.add_public_key("Bob", bob_key.verifying_key());
    blockchain.add_public_key("Charlie", charlie_key.verifying_key());

    // Stampa blocco genesis
    blockchain.print_block(&blockchain.blocks[0]);
    blockchain.print_balances();

    // Blocco 1: Transazione valida
    let tx1 = Transaction::new("Alice".to_string(), "Bob".to_string(), 30.0, &alice_key);
    blockchain.add_block(vec![tx1], authority.clone());
    blockchain.print_block(&blockchain.blocks[1]);
    blockchain.print_balances();

    // Blocco 2: Transazione con autorità non valida
    let tx2 = Transaction::new("Bob".to_string(), "Charlie".to_string(), 15.0, &bob_key);
    blockchain.add_block(vec![tx2], "Node2".to_string());
    blockchain.print_balances();

    // Blocco 3: Transazione con saldo insufficiente
    let tx3 = Transaction::new("Charlie".to_string(), "Alice".to_string(), 100.0, &charlie_key);
    blockchain.add_block(vec![tx3], authority.clone());
    blockchain.print_balances();

    // Blocco 4: Transazione valida
    let tx4 = Transaction::new("Charlie".to_string(), "Alice".to_string(), 5.0, &charlie_key);
    blockchain.add_block(vec![tx4], authority);
    blockchain.print_block(blockchain.blocks.last().unwrap());
    blockchain.print_balances();
}
