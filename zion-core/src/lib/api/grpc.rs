use tonic::{transport::Server, Request, Response, Status};
use zion_core::blockchain::Blockchain;
use std::sync::Arc;
use tokio::sync::Mutex;
use blockrock::transaction_service_server::{TransactionService, TransactionServiceServer};

pub mod blockrock {
    tonic::include_proto!("blockrock");
}

#[derive(Clone)]
pub struct TransactionServiceImpl {
    blockchain: Arc<Mutex<Blockchain>>,
}

#[tonic::async_trait]
impl TransactionService for TransactionServiceImpl {
    async fn get_transaction(
        &self,
        request: Request<GetTransactionRequest>,
    ) -> Result<Response<TransactionResponse>, Status> {
        let id = request.into_inner().id;
        let blockchain = self.blockchain.lock().await;
        // Presume che Blockchain abbia un metodo get_transaction
        let tx = blockchain.get_transaction(&id).ok_or(Status::not_found("Transazione non trovata"))?;
        Ok(Response::new(TransactionResponse {
            transaction: Some(blockrock::Transaction {
                id: tx.id,
                from: tx.from,
                to: tx.to,
                amount: tx.amount,
            }),
        }))
    }
}

pub async fn start_grpc(blockchain: Arc<Mutex<Blockchain>>, port: u16) -> Result<(), Box<dyn std::error::Error>> {
    let addr = format!("[::1]:{}", port).parse()?;
    let service = TransactionServiceImpl { blockchain };
    Server::builder()
        .add_service(TransactionServiceServer::new(service))
        .serve(addr)
        .await?;
    Ok(())
}
