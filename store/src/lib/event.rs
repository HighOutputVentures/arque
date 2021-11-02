use super::EventId;
use flatbuffers::FlatBufferBuilder;
use std::{
    convert::TryInto,
    time::{SystemTime, UNIX_EPOCH},
};

pub struct Event {
    pub buf: Vec<u8>,
}

pub struct EventArgs {
    pub r#type: String,
    pub aggregate_id: [u8; 16],
    pub aggregate_version: u64,
    pub data: Vec<u8>,
    pub meta: Vec<u8>,
}

pub struct EventReader<'a> {
    flatbuffer: super::fbs::event::Event<'a>,
}

impl<'a> EventReader<'a> {
    pub fn get_id(&self) -> EventId {
        EventId {
            buf: self.flatbuffer.id().unwrap().try_into().unwrap(),
        }
    }

    pub fn get_type(&self) -> &'a str {
        self.flatbuffer.type_().unwrap()
    }

    pub fn get_aggregate_id(&self) -> &'a [u8; 16] {
        self.flatbuffer.aggregate_id().unwrap().try_into().unwrap()
    }

    pub fn get_aggregate_version(&self) -> u64 {
        self.flatbuffer.aggregate_version()
    }
}

impl Event {
    pub fn new(args: EventArgs) -> Self {
        let mut buf = Vec::new();
        let mut fbb = FlatBufferBuilder::new();

        let id = EventId::new();

        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64;

        let event_args = super::fbs::event::EventArgs {
            id: Some(fbb.create_vector(&id.buf)),
            type_: Some(fbb.create_string(&args.r#type)),
            aggregate_id: Some(fbb.create_vector(&args.aggregate_id)),
            aggregate_version: args.aggregate_version,
            data: Some(fbb.create_vector(&args.data)),
            meta: Some(fbb.create_vector(&args.meta)),
            timestamp,
        };

        let root = super::fbs::event::Event::create(&mut fbb, &event_args);

        super::fbs::event::finish_event_buffer(&mut fbb, root);

        buf.extend_from_slice(fbb.finished_data());

        Event { buf }
    }

    pub fn as_reader<'a>(&'a self) -> EventReader<'a> {
        let flatbuffer = unsafe { super::fbs::event::root_as_event_unchecked(&self.buf[..]) };

        EventReader { flatbuffer }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_event_new() {
        let event = Event::new(EventArgs {
            r#type: "AccountCreated".to_string(),
            aggregate_id: [0; 16],
            aggregate_version: 1,
            data: vec![0; 32],
            meta: vec![0; 32],
        });

        let reader = event.as_reader();
        println!("{:?}", reader.get_id());
    }
}
