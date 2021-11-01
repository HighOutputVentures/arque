use crate::store::{CreateEventRequest};
use tonic::{Request};

async fn create_event(req: Request<CreateEventRequest>) {
  println!("{:#?}", req.get_ref());
}

#[cfg(test)]
mod tests {
    use prost_types::Struct;
    use std::collections::BTreeMap;

    use super::*;

    #[tokio::test]
    async fn test_create_event() {
        let req = Request::new(CreateEventRequest {
            r#type: "CreateAccount".to_string(),
            aggregate_id: vec![0; 16],
            aggregate_version: 1,
            data: Some(Struct {
                fields: BTreeMap::new()
            }),
            meta: Some(Struct {
                fields: BTreeMap::new()
            }),
        });

        create_event(req).await;
    }
}