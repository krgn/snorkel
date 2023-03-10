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

fn gen_flat_vec() -> Vec<FlatVecItem> {
    let mut rng = rand::thread_rng();
    let mut v = Vec::with_capacity(ROWS * COLS);
    for row in 0..ROWS {
        for col in 0..COLS {
            v.push(FlatVecItem {
                x: col,
                y: row,
                val: rng.gen_range(0..100),
            })
        }
    }
    return v;
}

fn gen_map() -> HashMap<(usize, usize), usize> {
    let mut rng = rand::thread_rng();
    let mut map = HashMap::with_capacity(ROWS * COLS);
    for row in 0..ROWS {
        for col in 0..COLS {
            map.insert((row, col), rng.gen_range(0..100));
        }
    }
    map
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

fn bench_flat_vec_lookup(data: &Vec<FlatVecItem>) {
    for (x, y) in COORDS {
        let _ = data.iter().find(|item| item.x == x && item.y == y).unwrap();
    }
}

fn bench_map_lookup(data: &HashMap<(usize, usize), usize>) {
    for (x, y) in COORDS {
        let _ = data.get(&(x, y)).unwrap();
    }
}

fn bench_2dvec_lookup(data: &Vec<Vec<usize>>) {
    for (x, y) in COORDS {
        let _ = data[y][x];
    }
}

fn flat_vec_benchmark(c: &mut Criterion) {
    let flat_vec = gen_flat_vec();
    c.bench_function("flat vec lookup", |b| {
        b.iter(|| bench_flat_vec_lookup(black_box(&flat_vec)))
    });
}

fn map_benchmark(c: &mut Criterion) {
    let map = gen_map();
    c.bench_function("map lookup", |b| {
        b.iter(|| bench_map_lookup(black_box(&map)))
    });
}

fn two_d_benchmark(c: &mut Criterion) {
    let two_dvec = gen_2dvec();
    c.bench_function("2d vec lookup", |b| {
        b.iter(|| bench_2dvec_lookup(black_box(&two_dvec)))
    });
}

criterion_group!(benches, flat_vec_benchmark);
criterion_main!(benches);
