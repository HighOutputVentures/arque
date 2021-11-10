use std::convert::TryInto;

use crate::lib::{Event, EventArgs, EventDb};
use crate::store::{CreateEventRequest, CreateEventResponse};
use tonic::{Request, Response, Status};

pub async fn create_event(
    db: &mut EventDb,
    req: Request<CreateEventRequest>,
) -> Result<Response<CreateEventResponse>, Status> {
    let req = req.get_ref();

    let event = Event::new(EventArgs {
        r#type: req.r#type.to_owned(),
        aggregate_id: req.aggregate_id.to_owned().try_into().unwrap(),
        aggregate_version: req.aggregate_version,
        data: &req.data,
        meta: &req.meta,
    });

    db.insert(event);

    Ok(Response::new(CreateEventResponse { error: None }))
}

#[cfg(test)]
mod tests {
    use super::{create_event, CreateEventRequest, EventDb, Request};
    use std::{path::Path, time::Instant};

    #[tokio::test]
    async fn test_create_event() {
        let req = Request::new(CreateEventRequest {
            r#type: "AccountCreated".to_string(),
            aggregate_id: vec![0; 16],
            aggregate_version: 1,
            data: vec![0; 1024],
            meta: vec![0; 128],
        });

        let mut db = EventDb::open(Path::new(".test"));

        let instant = Instant::now();
        create_event(&mut db, req).await;
        println!("{:?}", instant.elapsed().as_micros());

        std::fs::remove_dir_all(".test").unwrap();
    }
}
