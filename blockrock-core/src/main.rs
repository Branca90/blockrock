use sha2::{Digest, Sha256};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug)]
struct Block {
    index: u32,
    timestamp: u64,
    data: String,
    previous_hash: String,
    hash: String,
    authority: String,
}

impl Block {
    fn new(index: u32, data: String, previous_hash: String, authority: String) -> Block {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let hash = Block::calculate_hash(index, timestamp, &data, &previous_hash, &authority);
        Block {
            index,
            timestamp,
            data,
            previous_hash,
            hash,
            authority,
        }
    }

    fn calculate_hash(index: u32, timestamp: u64, data: &str, previous_hash: &str, authority: &str) -> String {
        let input = format!("{}{}{}{}{}", index, timestamp, data, previous_hash, authority);
        let mut hasher = Sha256::new();
        hasher.update(input);
        let result = hasher.finalize();
        hex::encode(result)
    }
}

struct Blockchain {
    blocks: Vec<Block>,
    authority: String,
}

impl Blockchain {
    fn new(authority: String) -> Blockchain {
        let genesis_block = Block::new(0, String::from("Genesis Block"), String::from("0"), authority.clone());
        Blockchain {
            blocks: vec![genesis_block],
            authority,
        }
    }

    fn add_block(&mut self, data: String, authority: String) -> bool {
        if authority != self.authority {
            println!("Errore: Autorità non valida! ({})", authority);
            return false;
        }
        let previous_block = self.blocks.last().unwrap();
        let new_block = Block::new(
            previous_block.index + 1,
            data,
            previous_block.hash.clone(),
            authority,
        );
        self.blocks.push(new_block);
        true
    }

    fn print_block(&self, block: &Block) {
        println!(
            "Block #{}: timestamp={}, data=\"{}\", previous_hash=\"{}\", hash=\"{}\", authority=\"{}\"",
            block.index,
            block.timestamp,
            block.data,
            block.previous_hash,
            block.hash,
            block.authority
        );
    }
}

fn main() {
    println!("BlockRock Blockchain Starting...");
    let authority = String::from("Node1");
    let mut blockchain = Blockchain::new(authority.clone());

    // Stampa il blocco genesis
    blockchain.print_block(&blockchain.blocks[0]);

    // Aggiungi il primo blocco
    blockchain.add_block(String::from("First Transaction"), authority.clone());
    blockchain.print_block(&blockchain.blocks[1]);

    // Prova con un'autorità sbagliata
    blockchain.add_block(String::from("Second Transaction"), String::from("Node2"));
    println!("Blockchain dopo tentativo non autorizzato:");
    for block in &blockchain.blocks {
        blockchain.print_block(block);
    }

    // Aggiungi il secondo blocco
    blockchain.add_block(String::from("Second Transaction"), authority);
    blockchain.print_block(&blockchain.blocks[2]);
}
