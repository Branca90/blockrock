use serde::{Serialize, Deserialize};
use ed25519_dalek::{SigningKey, VerifyingKey, Signer, Verifier, Signature};
use sha2::{Sha256, Digest};
use std::fmt;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub sender: String,
    pub receiver: String,
    pub amount: f64,
    pub signature: Option<Signature>,
}

impl Transaction {
    pub fn new(sender: String, receiver: String, amount: f64, signing_key: &SigningKey) -> Self {
        let mut transaction = Transaction {
            sender,
            receiver,
            amount,
            signature: None,
        };
        let signature = transaction.sign(signing_key);
        transaction.signature = Some(signature);
        transaction
    }

    fn sign(&self, signing_key: &SigningKey) -> Signature {
        let message = self.to_bytes();
        signing_key.sign(&message)
    }

    pub fn verify(&self, verifying_key: &VerifyingKey) -> bool {
        if let Some(signature) = &self.signature {
            let message = self.to_bytes();
            verifying_key.verify(&message, signature).is_ok()
        } else {
            false
        }
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut hasher = Sha256::new();
        hasher.update(&self.sender);
        hasher.update(&self.receiver);
        hasher.update(self.amount.to_le_bytes());
        hasher.finalize().to_vec()
    }
}

impl fmt::Display for Transaction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Transaction {{ sender: {}, receiver: {}, amount: {} }}",
            self.sender, self.receiver, self.amount
        )
    }
}
