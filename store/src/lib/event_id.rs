use machine_uid;

lazy_static! {
  static ref HOST_ID: String = {
    match machine_uid::get() {
      Ok(uid) => { uid },
      Err(e) => { panic!("failed to retrieve machine uid: {:?}", e) }
    }
  };
  static ref COUNTER: u32 = 0;
}

pub struct EventId {
  
}

impl EventId {
    pub fn new() -> Self {
      println!("{:#?}", *HOST_ID);
      Self {}
    }
}