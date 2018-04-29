#[macro_use]
extern crate bencher;
extern crate bettis;

use bencher::Bencher;
use bettis::Database;

fn database_set(bench: &mut Bencher) {
    bench.iter(|| {
        let mut database = Database::new();
        let value = bettis::bulk_string("example-1");

        database.set("example", value);
    })
}

fn database_get(bench: &mut Bencher) {
    let mut database = Database::new();
    let value = bettis::bulk_string("example-1");
    database.set("example", value);

    bench.iter(|| {
        database.get("example");
    })
}

fn database_incr(bench: &mut Bencher) {
    bench.iter(|| {
        let mut database = Database::new();
        let value = bettis::integer(1);
        let _ = database.incr("example");
    })
}

fn database_decr(bench: &mut Bencher) {
    bench.iter(|| {
        let mut database = Database::new();
        let value = bettis::integer(1);
        let _ = database.decr("example");
    })
}

benchmark_group!(benches,
    database_get,
    database_set,
    database_incr,
    database_decr
);

benchmark_main!(benches);