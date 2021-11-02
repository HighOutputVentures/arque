#[macro_use]
extern crate lazy_static;

mod store {
    tonic::include_proto!("store");
}
mod handlers;
mod lib;

use lib::EventDb;
use std::path::Path;
use store::store_server::{Store, StoreServer};
use store::{CreateEventRequest, CreateEventResponse};
use tonic::transport::Server;
use tonic::{Request, Response, Status};

pub struct StoreService {
    db: EventDb,
}

#[tonic::async_trait]
impl Store for StoreService {
    async fn create_event(
        &self,
        request: Request<CreateEventRequest>,
    ) -> Result<Response<CreateEventResponse>, Status> {
        let request = request.get_ref();
        println!("{:?}", request);

        unimplemented!()
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:22714".parse().unwrap();

    let service = StoreService {
        db: EventDb::open(Path::new(".data")),
    };

    let server = StoreServer::new(service);

    match Server::builder().add_service(server).serve(addr).await {
        Ok(_) => {}
        Err(e) => {
            panic!("failed to start server: {:?}", e)
        }
    };

    Ok(())
}
