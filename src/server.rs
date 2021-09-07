use tonic::transport::Server;

use storage_v1::StorageSvc1;
mod storage_v1;
use crate::storage_v1::storage_server::StorageServer as StorageServer1;

use storage_v2::StorageSvc2;
mod storage_v2;
use crate::storage_v2::storage_server::StorageServer as StorageServer2;

mod pool;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let svc1 = StorageSvc1::default();
    let svc2 = StorageSvc2::default();

    Server::builder()
        .add_service(StorageServer1::new(svc1))
        .add_service(StorageServer2::new(svc2))
        .
        .serve(addr)
        .await?;

    Ok(())
}
