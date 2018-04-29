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

/*
fn a(bench: &mut Bencher) {
    bench.iter(|| {
        (0..1000).fold(0, |x, y| x + y)
    })
}

fn b(bench: &mut Bencher) {
    const N: usize = 1024;
    bench.iter(|| {
        vec![0u8; N]
    });

    bench.bytes = N as u64;
}
*/

benchmark_group!(benches, database_set);
benchmark_main!(benches);