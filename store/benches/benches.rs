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

pub fn bench_event_id_new(c: &mut Criterion) {
    c.bench_function("EventId::new()", |b| b.iter(|| EventId::new()));
}

criterion_group!(benches, bench_event_id_new);
criterion_main!(benches);
