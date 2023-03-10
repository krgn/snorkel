use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rand::Rng;
use std::collections::HashMap;

const ROWS: usize = 10;
const COLS: usize = 20;

const COORDS: [(usize, usize); ROWS] = [
    (1, 5),
    (12, 4),
    (3, 7),
    (5, 8),
    (9, 3),
    (13, 1),
    (18, 3),
    (6, 5),
    (7, 2),
    (10, 2),
];

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct FlatVecItem {
    x: usize,
    y: usize,
    val: usize,
}

fn gen_map() -> HashMap<(usize, usize), usize> {
    let mut rng = rand::thread_rng();
    let mut map = HashMap::with_capacity(ROWS * COLS);
    for row in 0..ROWS {
        for col in 0..COLS {
            map.insert((col, row), rng.gen_range(0..100));
        }
    }
    map
}

fn bench_map_lookup(data: &HashMap<(usize, usize), usize>) {
    for (x, y) in COORDS {
        let _ = data.get(&(x, y)).unwrap();
    }
}

fn map_benchmark(c: &mut Criterion) {
    let map = gen_map();
    c.bench_function("map lookup", |b| {
        b.iter(|| bench_map_lookup(black_box(&map)))
    });
}

criterion_group!(benches, map_benchmark);
criterion_main!(benches);
