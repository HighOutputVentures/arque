use std::path::Path;
use leveldb::database::Database;
use leveldb::kv::KV;
use leveldb::options::{Options, WriteOptions};
use std::time::Instant;


struct EventDb {}

impl EventDb {
  fn open(path: &Path) {
    let mut options = Options::new();
    options.create_if_missing = true;
    let db = Database::open(path, options).unwrap();

    let write_opts = WriteOptions::new();
    let instant = Instant::now();
    for n in 0..1000000 {
      db.put(write_opts, n, &[1]).unwrap();
    }
    println!("{:?}", instant.elapsed());
  }
}

mod tests {
  use super::*;

  #[test]
  fn generate_unique() {
    let path = Path::new("/tmp/event_db");
    EventDb::open(path);
  }
}