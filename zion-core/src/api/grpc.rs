use std::sync::Arc;
use tokio::sync::Mutex;
use tonic::{transport::Server, Request, Response, Status};
use blockrock_core::blockchain::Blockchain;
use blockrock::transaction_service_server::{TransactionService, TransactionServiceServer};
use blockrock::{TransactionRequest, TransactionResponse};

pub mod blockrock {
    tonic::include_proto!("blockrock");
}

#[derive(Clone)]
pub struct MyTransactionService {
    blockchain: Arc<Mutex<Blockchain>>,
}

#[tonic::async_trait]
impl TransactionService for MyTransactionService {
    async fn get_transaction(
        &self,
        request: Request<TransactionRequest>,
    ) -> Result<Response<TransactionResponse>, Status> {
        let id = request.into_inner().id;
        let blockchain = self.blockchain.lock().await;
        // Placeholder: cerca una transazione nella blockchain
        let balances = blockchain.get_balances(); // Usa blockchain per qualcosa
        let found = balances.iter().any(|(addr, _)| addr == &id);
        if found {
            Ok(Response::new(TransactionResponse {
                id,
                sender: "sender".to_string(),
                recipient: "recipient".to_string(),
                amount: 100.0,
            }))
        } else {
            Err(Status::not_found("Transaction not found"))
        }
    }
}

pub async fn start_grpc(blockchain: Arc<Mutex<Blockchain>>, port: u16) -> Result<(), anyhow::Error> {
    let addr = format!("0.0.0.0:{}", port).parse()?;
    let service = MyTransactionService { blockchain };
    Server::builder()
        .add_service(TransactionServiceServer::new(service))
        .serve(addr)
        .await?;
    Ok(())
}
