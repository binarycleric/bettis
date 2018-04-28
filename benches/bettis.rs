#[macro_use]
extern crate criterion;
extern crate bettis;
extern crate resp;

use criterion::Criterion;
use bettis::storage::Database;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("database.get", |b| {
        let mut database = Database::new();
        database.set("example", resp::Value::Integer(99));

        b.iter(|| {
            let _ = database.get("example");
        });
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);