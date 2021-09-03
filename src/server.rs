use tonic::{transport::Server, Request, Response, Status};

use storage::storage_server::{Storage, StorageServer};
use storage::{CreatePoolRequest, Pool};

pub mod storage {
    tonic::include_proto!("storage");
}


#[derive(Debug, Default)]
pub struct MyStorage {}

#[tonic::async_trait]
impl Storage for MyStorage {
    async fn create_pool(
        &self,
        request: Request<CreatePoolRequest>,
    ) -> Result<Response<Pool>, Status> { 
        println!("Got a request: {:?}", request);

        let reply = storage::Pool {
            name: request.into_inner().name,
        };

        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let storage = MyStorage::default();

    Server::builder()
        .add_service(StorageServer::new(storage))
        .serve(addr)
        .await?;

    Ok(())
}
