// src/grpc.rs
use std::sync::Arc;
use tokio::sync::Mutex;
use blockrock_core::blockchain::Blockchain;

pub async fn start_grpc_server(blockchain: Arc<Mutex<Blockchain>>, port: u16) -> Result<(), Box<dyn std::error::Error>> {
    // Implementazione placeholder per gRPC
    tokio::time::sleep(tokio::time::Duration::from_secs(10)).await; // Simula un server gRPC
    Ok(())
}
