#[macro_use]
extern crate lazy_static;

mod store {
    tonic::include_proto!("store");
}
mod lib;

use tonic::{transport::Server};
use store::store_server::{Store, StoreServer};
use store::{CreateEventRequest, CreateEventResponse};
use tonic::{Request, Response, Status};
use lib::event_id::EventId;

#[derive(Debug)]
#[derive(Default)]
pub struct StoreService {}

#[tonic::async_trait]
impl Store for StoreService {
  async fn create_event(&self, request: Request<CreateEventRequest>) -> Result<Response<CreateEventResponse>, Status> {
    request.get_ref();
    unimplemented!()
  }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:10000".parse().unwrap();

    let store = StoreService::default();

    let server = StoreServer::new(store);

    EventId::new();

    match Server::builder().add_service(server).serve(addr).await {
      Ok(_) => {},
      Err(e) => { panic!("failed to start server: {:?}", e) }
    };

    Ok(())
}
