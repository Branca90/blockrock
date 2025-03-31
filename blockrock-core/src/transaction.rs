use ed25519_dalek::{Signer, Signature, SigningKey, VerifyingKey};
use ed25519_dalek::Verifier;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Transaction {
    pub sender: String,
    pub receiver: String,
    pub amount: f64,
    pub signature: Option<Signature>,
}

impl Transaction {
    pub fn new(sender: String, receiver: String, amount: f64, signing_key: &SigningKey) -> Self {
        let mut tx = Self {
            sender,
            receiver,
            amount,
            signature: None,
        };
        tx.sign(signing_key);
        tx
    }

    pub fn to_string(&self) -> String {
        format!("{} -> {}: {}", self.sender, self.receiver, self.amount)
    }

    fn sign(&mut self, signing_key: &SigningKey) {
        let message_str = self.to_string();
        let message = message_str.as_bytes();
        self.signature = Some(signing_key.sign(message));
    }

    pub fn verify(&self, public_key: &VerifyingKey) -> bool {
        let message_str = self.to_string();
        let message = message_str.as_bytes();
        self.signature
            .as_ref()
            .map_or(false, |sig| public_key.verify(message, sig).is_ok())
    }
}
