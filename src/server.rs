use tonic::{transport::Server, Request, Response, Status};

use storage::storage_server::{Storage, StorageServer};
use storage::{CreatePoolRequest, DestroyPoolRequest, ListPoolsReply, Null, Pool, PoolType};

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
        println!("Got a create request: {:?}", request);

        let args = request.into_inner();

        let reply = Pool {
            name: args.name,
            disk: args.disk,
            pooltype: args.pooltype,
        };
        Ok(Response::new(reply))
    }

    async fn destroy_pool(
        &self,
        request: Request<DestroyPoolRequest>,
    ) -> Result<Response<Null>, Status> {
        println!("Got a destroy request: {:?}", request);

        let _args = request.into_inner();

        let reply = Null {};
        Ok(Response::new(reply))
    }

    async fn list_pools(&self, request: Request<Null>) -> Result<Response<ListPoolsReply>, Status> {
        println!("Got a list request: {:?}", request);

        let reply = ListPoolsReply {
            pools: vec![Pool {
                name: "tpool".to_string(),
                disk: vec![],
                pooltype: PoolType::Lvm as i32,
            }],
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
