use criterion::*;
use std::time::Instant;


pub fn iteration_aos(c: &mut Criterion) {
    let block_size = 10_000;

    let mut group = c.benchmark_group("baseline-array-of-structs");
    group.bench_with_input(BenchmarkId::new("iteration", 1), &1, |bencher, _| {
        let mut vec: Vec<_> = (0..block_size).map(|_| (0u32,)).collect();
        bencher.iter_custom(|iters| {
            let start = Instant::now();
            vec.iter().cycle().take(iters as usize).for_each(|inner| { black_box(*inner); });
            Instant::now().duration_since(start)
        })
    });
    group.bench_with_input(BenchmarkId::new("iteration", 2), &2, |bencher, _| {
        let mut vec: Vec<_> = (0..block_size).map(|_| (0u32, 0u32)).collect();
        bencher.iter_custom(|iters| {
            let start = Instant::now();
            vec.iter().cycle().take(iters as usize).for_each(|inner| { black_box(*inner); });
            Instant::now().duration_since(start)
        })
    });
    group.bench_with_input(BenchmarkId::new("iteration", 3), &3, |bencher, _| {
        let mut vec: Vec<_> = (0..block_size).map(|_| (0u32, 0u32, 0u32)).collect();
        bencher.iter_custom(|iters| {
            let start = Instant::now();
            vec.iter().cycle().take(iters as usize).for_each(|inner| { black_box(*inner); });
            Instant::now().duration_since(start)
        })
    });
    group.bench_with_input(BenchmarkId::new("iteration", 4), &4, |bencher, _| {
        let mut vec: Vec<_> = (0..block_size).map(|_| (0u32, 0u32, 0u32, 0u32)).collect();
        bencher.iter_custom(|iters| {
            let start = Instant::now();
            vec.iter().cycle().take(iters as usize).for_each(|inner| { black_box(*inner); });
            Instant::now().duration_since(start)
        })
    });
    group.bench_with_input(BenchmarkId::new("iteration", 5), &5, |bencher, _| {
        let mut vec: Vec<_> = (0..block_size).map(|_| (0u32, 0u32, 0u32, 0u32, 0u32)).collect();
        bencher.iter_custom(|iters| {
            let start = Instant::now();
            vec.iter().cycle().take(iters as usize).for_each(|inner| { black_box(*inner); });
            Instant::now().duration_since(start)
        })
    });
    group.bench_with_input(BenchmarkId::new("iteration", 6), &6, |bencher, _| {
        let mut vec: Vec<_> = (0..block_size).map(|_| (0u32, 0u32, 0u32, 0u32, 0u32, 0u32)).collect();
        bencher.iter_custom(|iters| {
            let start = Instant::now();
            vec.iter().cycle().take(iters as usize).for_each(|inner| { black_box(*inner); });
            Instant::now().duration_since(start)
        })
    });
    group.finish();
}

pub fn iteration_soa(c: &mut Criterion) {
    let block_size = 10_000;

    let mut group = c.benchmark_group("baseline-structs-of-arrays");
    group.bench_with_input(BenchmarkId::new("iteration", 1), &1, |bencher, _| {
        let mut a: Vec<_> = (0..block_size).map(|_| (0u32,)).collect();
        bencher.iter_custom(|iters| {
            let start = Instant::now();
            a.iter().cycle().take(iters as usize).for_each(|inner| { black_box(*inner); });
            Instant::now().duration_since(start)
        })
    });
    group.bench_with_input(BenchmarkId::new("iteration", 2), &2, |bencher, _| {
        let mut a: Vec<_> = (0..block_size).map(|_| (0u32,)).collect();
        let mut b: Vec<_> = (0..block_size).map(|_| (0u32,)).collect();
        bencher.iter_custom(|iters| {
            let start = Instant::now();
            a.iter().map(|x| black_box(*x))
                .zip(b.iter().map(|x| black_box(*x)))
                .cycle()
                .take(iters as usize)
                .for_each(|_| ());
            Instant::now().duration_since(start)
        })
    });
    group.bench_with_input(BenchmarkId::new("iteration", 3), &3, |bencher, _| {
        let mut a: Vec<_> = (0..block_size).map(|_| (0u32,)).collect();
        let mut b: Vec<_> = (0..block_size).map(|_| (0u32,)).collect();
        let mut c: Vec<_> = (0..block_size).map(|_| (0u32,)).collect();
        bencher.iter_custom(|iters| {
            let start = Instant::now();
            a.iter().map(|x| black_box(*x))
                .zip(b.iter().map(|x| black_box(*x)))
                .zip(c.iter().map(|x| black_box(*x)))
                .cycle()
                .take(iters as usize)
                .for_each(|_| ());
            Instant::now().duration_since(start)
        })
    });
    group.bench_with_input(BenchmarkId::new("iteration", 4), &4, |bencher, _| {
        let mut a: Vec<_> = (0..block_size).map(|_| (0u32,)).collect();
        let mut b: Vec<_> = (0..block_size).map(|_| (0u32,)).collect();
        let mut c: Vec<_> = (0..block_size).map(|_| (0u32,)).collect();
        let mut d: Vec<_> = (0..block_size).map(|_| (0u32,)).collect();
        bencher.iter_custom(|iters| {
            let start = Instant::now();
            a.iter().map(|x| black_box(*x))
                .zip(b.iter().map(|x| black_box(*x)))
                .zip(c.iter().map(|x| black_box(*x)))
                .zip(d.iter().map(|x| black_box(*x)))
                .cycle()
                .take(iters as usize)
                .for_each(|_| ());
            Instant::now().duration_since(start)
        })
    });
    group.bench_with_input(BenchmarkId::new("iteration", 5), &5, |bencher, _| {
        let mut a: Vec<_> = (0..block_size).map(|_| (0u32,)).collect();
        let mut b: Vec<_> = (0..block_size).map(|_| (0u32,)).collect();
        let mut c: Vec<_> = (0..block_size).map(|_| (0u32,)).collect();
        let mut d: Vec<_> = (0..block_size).map(|_| (0u32,)).collect();
        let mut e: Vec<_> = (0..block_size).map(|_| (0u32,)).collect();
        bencher.iter_custom(|iters| {
            let start = Instant::now();
            a.iter().map(|x| black_box(*x))
                .zip(b.iter().map(|x| black_box(*x)))
                .zip(c.iter().map(|x| black_box(*x)))
                .zip(d.iter().map(|x| black_box(*x)))
                .zip(e.iter().map(|x| black_box(*x)))
                .cycle()
                .take(iters as usize)
                .for_each(|_| ());
            Instant::now().duration_since(start)
        })
    });
    group.bench_with_input(BenchmarkId::new("iteration", 6), &6, |bencher, _| {
        let mut a: Vec<_> = (0..block_size).map(|_| (0u32,)).collect();
        let mut b: Vec<_> = (0..block_size).map(|_| (0u32,)).collect();
        let mut c: Vec<_> = (0..block_size).map(|_| (0u32,)).collect();
        let mut d: Vec<_> = (0..block_size).map(|_| (0u32,)).collect();
        let mut e: Vec<_> = (0..block_size).map(|_| (0u32,)).collect();
        let mut f: Vec<_> = (0..block_size).map(|_| (0u32,)).collect();
        bencher.iter_custom(|iters| {
            let start = Instant::now();
            a.iter().map(|x| black_box(*x))
                .zip(b.iter().map(|x| black_box(*x)))
                .zip(c.iter().map(|x| black_box(*x)))
                .zip(d.iter().map(|x| black_box(*x)))
                .zip(e.iter().map(|x| black_box(*x)))
                .zip(f.iter().map(|x| black_box(*x)))
                .cycle()
                .take(iters as usize)
                .for_each(|_| ());
            Instant::now().duration_since(start)
        })
    });
    group.finish();
}

criterion_group!(
    basic,
    iteration_aos,
    iteration_soa
);
criterion_main!(basic);