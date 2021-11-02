use bytes::BufMut;
use db_key::Key;
use rand::random;
use std::collections::hash_map::DefaultHasher;
use std::convert::TryInto;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};

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
            process_id: std::process::id(),
        }
        .hash(&mut hasher);

        hasher.finish()
    };
    static ref COUNTER: Arc<Mutex<u32>> = Arc::new(Mutex::new(random()));
}

#[derive(PartialEq, Debug)]
pub struct EventId {
    pub buf: [u8; 12],
}

impl EventId {
    pub fn new() -> Self {
        let mut dst = [0; 12];

        let mut buf = &mut dst[..];

        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as u32;

        buf.put_u32(timestamp);

        buf.put(&HOST_ID.to_be_bytes()[..5]);

        buf.put(&COUNTER.lock().unwrap().to_be_bytes()[1..]);

        let mut counter = COUNTER.lock().unwrap();
        *counter += 1;

        Self { buf: dst }
    }
}

impl Key for EventId {
    fn from_u8(key: &[u8]) -> Self {
        assert!(key.len() == 12);

        Self {
            buf: key.to_owned().try_into().unwrap(),
        }
    }
    fn as_slice<T, F: Fn(&[u8]) -> T>(&self, f: F) -> T {
        f(&self.buf)
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
