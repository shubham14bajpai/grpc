use tonic::{Request, Response, Status};
tonic::include_proto!("storage.v2");

use crate::pool::{
    MyPool,
    create_pool,
};

impl From<MyPool> for Pool {
    fn from(p: MyPool) -> Self {
        Self {
            name: p.name,
            disk: p.disks,
            pooltype: p.pooltype,
        }
    }
}

#[derive(Debug, Default)]
pub struct StorageSvc2 {}

#[tonic::async_trait]
impl storage_server::Storage for StorageSvc2 {
    async fn create_pool(
        &self,
        request: Request<CreatePoolRequest>,
    ) -> Result<Response<Pool>, Status> {
        println!("Got a create request: {:?}", request);

        let args = request.into_inner();
        
        let reply = create_pool(MyPool{
            name: args.name,
            disks: args.disk,
            pooltype: args.pooltype as i32,
        });
        Ok(Response::new(Pool::from(reply)))
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
