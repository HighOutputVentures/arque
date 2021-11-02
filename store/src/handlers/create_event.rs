use std::convert::TryInto;

use crate::lib::{Event, EventArgs, EventDb};
use crate::store::CreateEventRequest;
use tonic::Request;

pub async fn create_event(db: &mut EventDb, req: Request<CreateEventRequest>) {
    let req = req.get_ref();

    let event = Event::new(EventArgs {
        r#type: req.r#type.to_owned(),
        aggregate_id: req.aggregate_id.to_owned().try_into().unwrap(),
        aggregate_version: req.aggregate_version,
        data: req.data.to_owned(),
        meta: req.meta.to_owned(),
    });

    db.insert(event);
}

#[cfg(test)]
mod tests {
    use super::{create_event, CreateEventRequest, EventDb, Request};
    use std::path::Path;

    #[tokio::test]
    async fn test_create_event() {
        let req = Request::new(CreateEventRequest {
            r#type: "AccountCreated".to_string(),
            aggregate_id: vec![0; 16],
            aggregate_version: 1,
            data: vec![0; 32],
            meta: vec![0; 32],
        });

        let mut db = EventDb::open(Path::new(".test"));

        create_event(&mut db, req).await;

        std::fs::remove_dir_all(".test").unwrap();
    }
}
