use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rand::Rng;

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

fn gen_2dvec() -> Vec<Vec<usize>> {
    let mut rng = rand::thread_rng();
    let mut rows = Vec::with_capacity(ROWS);
    for _ in 0..ROWS {
        let mut col = Vec::with_capacity(COLS);
        for _ in 0..COLS {
            col.push(rng.gen_range(0..100));
        }
        rows.push(col);
    }
    rows
}

fn bench_2dvec_lookup(data: &Vec<Vec<usize>>) {
    for (x, y) in COORDS {
        let _ = data[y][x];
    }
}

fn two_d_benchmark(c: &mut Criterion) {
    let two_dvec = gen_2dvec();
    c.bench_function("2d vec lookup", |b| {
        b.iter(|| bench_2dvec_lookup(black_box(&two_dvec)))
    });
}

criterion_group!(benches, two_d_benchmark);
criterion_main!(benches);
