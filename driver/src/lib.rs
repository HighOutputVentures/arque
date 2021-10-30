mod store {
    tonic::include_proto!("store");
}

use std::vec;

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
    options: Options,
    client: StoreClient<Channel>
}

impl Driver {
    pub async fn open(options: Options) -> Self {
        let client = match StoreClient::connect(format!("{}:{}", options.hostname, options.port)).await {
            Ok(client) => client,
            Err(e) => panic!("unable to connect to server: {}", e)
        };

        Driver {
            options,
            client
        }
    }

    pub async fn create_event() {
        let _req = Request::new(CreateEventRequest {
            r#type: "".to_string(),
            aggregate_id: vec![],
            aggregate_version: 0,
            version: 0,
            data: 
        });
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
