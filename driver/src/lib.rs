mod store {
    tonic::include_proto!("store");
}

pub use prost_types::{Struct, Value};
use store::store_client::{StoreClient};
use store::{CreateEventRequest};
use tonic::transport::Channel;
use tonic::{Request};

pub struct Options {
    pub port: u32,
    pub hostname: String,
}

impl Options {
    fn new() -> Self {
        Options {
            port: 22714,
            hostname: "[::1]".to_string()
        }
    }
}

pub struct Driver {
    client: StoreClient<Channel>
}

pub struct CreateEventParams {
    r#type: String,
    aggregate_id: [u8; 16],
    aggregate_version: u64,
    data: Struct,
    meta: Struct,
}

impl Driver {
    pub async fn open(options: Options) -> Result<Self, Box<dyn std::error::Error>> {
        let client = StoreClient::connect(format!("{}:{}", options.hostname, options.port)).await?;

        Ok(Driver {
            client
        })
    }

    pub async fn create_event(&mut self, params: CreateEventParams) -> Result<(), Box<dyn std::error::Error>> {
        let req = Request::new(CreateEventRequest {
            r#type: params.r#type,
            aggregate_id: params.aggregate_id.to_vec(),
            aggregate_version: params.aggregate_version,
            data: Some(params.data),
            meta: Some(params.meta),
        });

        self.client.create_event(req).await?;

        Ok(())
    }
}
