#[macro_use]
extern crate lazy_static;
#[path = "../src/handlers/mod.rs"]
mod handlers;
#[path = "../src/lib/mod.rs"]
mod lib;
mod store {
    tonic::include_proto!("store");
}

use criterion::{criterion_group, criterion_main, Criterion};
use lib::EventId;

pub fn event_id_new(c: &mut Criterion) {
    c.bench_function("EventId::new()", |b| b.iter(|| EventId::new()));
}

pub fn create_event(c: &mut Criterion) {
    c.bench_function("create_event(..)", |b| b.iter(|| EventId::new()));
}

criterion_group!(benches, event_id_new, create_event);
criterion_main!(benches);
