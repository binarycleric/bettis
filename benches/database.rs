#[macro_use]
extern crate bencher;
extern crate bettis;
extern crate rand;
extern crate resp;
extern crate chrono;

use bencher::Bencher;
use rand::{thread_rng, Rng};
use chrono::Duration;
use bettis::storage::Database;

fn bulk_string<'a>(string: &'a str) -> resp::Value {
    resp::Value::Bulk(string.to_string())
}

fn database_set(bench: &mut Bencher) {
    let mut database = Database::new();
    let value = bulk_string("example-1");

    bench.iter(|| {
        database.set("example".to_string(), value.clone());
    })
}

fn database_get(bench: &mut Bencher) {
    let mut database = Database::new();
    let value = bulk_string("example-1");
    database.set("example".to_string(), value);

    bench.iter(|| {
        database.get("example".to_string());
    })
}

fn database_get_missing_key(bench: &mut Bencher) {
    let mut database = Database::new();

    bench.iter(|| {
        database.get("example".to_string());
    })
}

fn database_get_with_ttled_key(bench: &mut Bencher) {
    let mut database = Database::new();
    let value = bulk_string("example-1");
    let _ = database.set("example".to_string(), value);
    let _ = database.set_ttl("example".to_string(), Duration::weeks(52));

    bench.iter(|| {
        database.get("example".to_string());
    })
}

fn database_get_with_expired_key(bench: &mut Bencher) {
    let mut database = Database::new();
    let value = bulk_string("example-1");
    let _ = database.set("example".to_string(), value);
    let _ = database.set_ttl("example".to_string(), Duration::microseconds(0));

    bench.iter(|| {
        database.get("example".to_string());
    })
}

fn database_get_with_10000_items(bench: &mut Bencher) {
    let mut rng = thread_rng();
    let mut database = Database::new();

    for x in 0..10000 {
        let value = bulk_string("example");
        database.set(format!("example-{}", x), value);
    }

    bench.iter(|| {
        let random_key = rng.gen_range(0..10000);
        database.get(format!("example-{}", random_key));
    })
}

benchmark_group!(
    benches,
    database_get,
    database_get_missing_key,
    database_get_with_10000_items,
    database_get_with_expired_key,
    database_get_with_ttled_key,
    database_set,
);

benchmark_main!(benches);
