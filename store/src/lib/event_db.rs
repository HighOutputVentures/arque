use super::{Event, EventId};
use leveldb::database::Database;
use leveldb::kv::KV;
use leveldb::options::{Options, WriteOptions};
use std::path::Path;

pub struct EventDb {
    db: Database<EventId>,
}

impl EventDb {
    pub fn open(path: &Path) -> EventDb {
        let mut options = Options::new();
        options.create_if_missing = true;
        let db = Database::open(path, options).unwrap();

        EventDb { db }
    }

    pub fn insert(&mut self, event: Event) {
        let options = WriteOptions::new();

        let reader = event.as_reader();

        self.db.put(options, &reader.get_id(), &event.buf).unwrap();
    }

    pub fn get() {}
}

mod tests {
    use super::*;

    #[test]
    fn generate_unique() {
        let path = Path::new("/tmp/event_db");
        EventDb::open(path);
    }
}
