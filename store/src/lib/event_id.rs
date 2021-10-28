use rand::random;
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;
use std::time::{SystemTime, UNIX_EPOCH};
use bytes::BufMut;
use std::sync::{Arc, Mutex};
use std::fmt;

#[derive(Hash)]
struct HostId {
    machine_uid: String,
    process_id: u32,
}

lazy_static! {
    static ref HOST_ID: u64 = {
        let mut hasher = DefaultHasher::new();

        HostId {
            machine_uid: machine_uid::get().unwrap(),
            process_id: std::process::id()
        }.hash(&mut hasher);

        hasher.finish()
    };

    static ref COUNTER: Arc<Mutex<u32>> = Arc::new(Mutex::new(random()));
}

#[derive(PartialEq, Debug)]
pub struct EventId {
  buf: [u8; 12]
}

impl EventId {
    pub fn new() -> Self {
        let mut dst = [0; 12];

        let mut buf = &mut dst[..];

        buf.put_u32(SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as u32);

        buf.put(&HOST_ID.to_be_bytes()[..5]);

        buf.put(&COUNTER.lock().unwrap().to_be_bytes()[1..]);

        let mut counter = COUNTER.lock().unwrap();
        *counter += 1;

        Self {
            buf: dst
        }
    }
}

impl fmt::Display for EventId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", hex::encode(self.buf))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_unique() {
        assert_ne!(EventId::new(), EventId::new());
    }
}