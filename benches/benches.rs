use criterion::{criterion_group, criterion_main, BatchSize, Criterion};
use kvs::{KvStore, KvsEngine, SledKvsStore};
use rand::prelude::*;
use tempfile::TempDir;

// Benchmark result:
//
// set_bench/kvs           time:   [199.71 µs 201.39 µs 203.43 µs]                          
// set_bench/sled          time:   [152.64 ms 154.90 ms 157.06 ms]                           
//
// get_bench/kvs_4         time:   [1.0545 µs 1.0557 µs 1.0569 µs]                             
// get_bench/kvs_8         time:   [1.0529 µs 1.0536 µs 1.0544 µs]                             
// get_bench/kvs_12        time:   [1.1336 µs 1.1350 µs 1.1368 µs]                              
// get_bench/kvs_16        time:   [1.2480 µs 1.2691 µs 1.2929 µs]                              
// get_bench/sled_4        time:   [584.00 ns 598.51 ns 613.50 ns]                             
// get_bench/sled_8        time:   [657.98 ns 675.58 ns 692.46 ns]
//
// Observation:
// 1. sled set is much slower than kvs. Probably because sled uses B+ tree, 
// whereas kvs is append only.
// 2. sled get is very fast. Two possible reasons: (1) kvs code is not highly
// optimized for performance; (2) kvs never optimizes how entries are stored on
// the disk, whereas sled organizes (sorted) data in a tree.


fn set_bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("set_bench");
    group.bench_function("kvs", |b| {
        b.iter_batched(
            || {
                let temp_dir = TempDir::new().unwrap();
                (KvStore::open(temp_dir.path()).unwrap(), temp_dir)
            },
            |(mut store, _temp_dir)| {
                for i in 1..(1 << 3) {
                    store.set(format!("key{}", i), "value".to_string()).unwrap();
                }
            },
            BatchSize::SmallInput,
        )
    });
    group.bench_function("sled", |b| {
        b.iter_batched(
            || {
                let temp_dir = TempDir::new().unwrap();
                (SledKvsStore::open(temp_dir.path()).unwrap(), temp_dir)
            },
            |(mut db, _temp_dir)| {
                for i in 1..(1 << 3) {
                    db.set(format!("key{}", i), "value".to_string()).unwrap();
                }
            },
            BatchSize::SmallInput,
        )
    });
    group.finish();
}

fn get_bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("get_bench");
    for i in &vec![4, 8, 12, 16] {
        group.bench_with_input(format!("kvs_{}", i), i, |b, i| {
            let temp_dir = TempDir::new().unwrap();
            let mut store = KvStore::open(temp_dir.path()).unwrap();
            for key_i in 1..(1 << i) {
                store
                    .set(format!("key{}", key_i), "value".to_string())
                    .unwrap();
            }
            let mut rng = SmallRng::from_seed([0; 16]);
            b.iter(|| {
                store
                    .get(format!("key{}", rng.gen_range(1, 1 << i)))
                    .unwrap();
            })
        });
    }
    for i in &vec![4, 8, 12, 16] {
        group.bench_with_input(format!("sled_{}", i), i, |b, i| {
            let temp_dir = TempDir::new().unwrap();
            let mut db = SledKvsStore::open(temp_dir.path()).unwrap();
            for key_i in 1..(1 << i) {
                db.set(format!("key{}", key_i), "value".to_string())
                    .unwrap();
            }
            let mut rng = SmallRng::from_seed([0; 16]);
            b.iter(|| {
                db.get(format!("key{}", rng.gen_range(1, 1 << i))).unwrap();
            })
        });
    }
    group.finish();
}

criterion_group!(benches, set_bench, get_bench);
criterion_main!(benches);
