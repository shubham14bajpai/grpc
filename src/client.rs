use storage::storage_client::StorageClient;
use storage::CreatePoolRequest;

pub mod storage {
    tonic::include_proto!("storage");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = StorageClient::connect("http://[::1]:50051").await?;

    let request = tonic::Request::new(CreatePoolRequest {
        name: "tpool".into(),
    });

    let response = client.create_pool(request).await?;

    println!("RESPONSE={:?}", response);

    Ok(())
}
