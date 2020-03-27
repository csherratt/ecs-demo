use criterion::*;
use std::time::Instant;
use criterion::measurement::WallTime;
use crate::suits::BLOCK_SIZE;

pub fn iteration_aos(group: &mut BenchmarkGroup<WallTime>) {
    group.bench_with_input(BenchmarkId::new("vec-aos", 1), &1, |bencher, _| {
        let vec: Vec<_> = (0..BLOCK_SIZE).map(|_| (0u32,)).collect();
        bencher.iter_custom(|iters| {
            let start = Instant::now();
            vec.iter().cycle().take(iters as usize).for_each(|inner| { black_box(*inner); });
            Instant::now().duration_since(start)
        })
    });
    group.bench_with_input(BenchmarkId::new("vec-aos", 2), &2, |bencher, _| {
        let vec: Vec<_> = (0..BLOCK_SIZE).map(|_| (0u32, 0u32)).collect();
        bencher.iter_custom(|iters| {
            let start = Instant::now();
            vec.iter().cycle().take(iters as usize).for_each(|inner| { black_box(*inner); });
            Instant::now().duration_since(start)
        })
    });
    group.bench_with_input(BenchmarkId::new("vec-aos", 3), &3, |bencher, _| {
        let vec: Vec<_> = (0..BLOCK_SIZE).map(|_| (0u32, 0u32, 0u32)).collect();
        bencher.iter_custom(|iters| {
            let start = Instant::now();
            vec.iter().cycle().take(iters as usize).for_each(|inner| { black_box(*inner); });
            Instant::now().duration_since(start)
        })
    });
    group.bench_with_input(BenchmarkId::new("vec-aos", 4), &4, |bencher, _| {
        let vec: Vec<_> = (0..BLOCK_SIZE).map(|_| (0u32, 0u32, 0u32, 0u32)).collect();
        bencher.iter_custom(|iters| {
            let start = Instant::now();
            vec.iter().cycle().take(iters as usize).for_each(|inner| { black_box(*inner); });
            Instant::now().duration_since(start)
        })
    });
    group.bench_with_input(BenchmarkId::new("vec-aos", 5), &5, |bencher, _| {
        let vec: Vec<_> = (0..BLOCK_SIZE).map(|_| (0u32, 0u32, 0u32, 0u32, 0u32)).collect();
        bencher.iter_custom(|iters| {
            let start = Instant::now();
            vec.iter().cycle().take(iters as usize).for_each(|inner| { black_box(*inner); });
            Instant::now().duration_since(start)
        })
    });
    group.bench_with_input(BenchmarkId::new("vec-aos", 6), &6, |bencher, _| {
        let vec: Vec<_> = (0..BLOCK_SIZE).map(|_| (0u32, 0u32, 0u32, 0u32, 0u32, 0u32)).collect();
        bencher.iter_custom(|iters| {
            let start = Instant::now();
            vec.iter().cycle().take(iters as usize).for_each(|inner| { black_box(*inner); });
            Instant::now().duration_since(start)
        })
    });
}

pub fn iteration_soa(group: &mut BenchmarkGroup<WallTime>) {
    let block_size = 10_000;
    group.bench_with_input(BenchmarkId::new("vec-soa", 1), &1, |bencher, _| {
        let a: Vec<_> = (0..block_size).map(|_| (0u32,)).collect();
        bencher.iter_custom(|iters| {
            let start = Instant::now();
            a.iter().cycle().take(iters as usize).for_each(|inner| { black_box(*inner); });
            Instant::now().duration_since(start)
        })
    });
    group.bench_with_input(BenchmarkId::new("vec-soa", 2), &2, |bencher, _| {
        let a: Vec<_> = (0..block_size).map(|_| (0u32,)).collect();
        let b: Vec<_> = (0..block_size).map(|_| (0u32,)).collect();
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
    group.bench_with_input(BenchmarkId::new("vec-soa", 3), &3, |bencher, _| {
        let a: Vec<_> = (0..block_size).map(|_| (0u32,)).collect();
        let b: Vec<_> = (0..block_size).map(|_| (0u32,)).collect();
        let c: Vec<_> = (0..block_size).map(|_| (0u32,)).collect();
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
    group.bench_with_input(BenchmarkId::new("vec-soa", 4), &4, |bencher, _| {
        let a: Vec<_> = (0..block_size).map(|_| (0u32,)).collect();
        let b: Vec<_> = (0..block_size).map(|_| (0u32,)).collect();
        let c: Vec<_> = (0..block_size).map(|_| (0u32,)).collect();
        let d: Vec<_> = (0..block_size).map(|_| (0u32,)).collect();
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
    group.bench_with_input(BenchmarkId::new("vec-soa", 5), &5, |bencher, _| {
        let a: Vec<_> = (0..block_size).map(|_| (0u32,)).collect();
        let b: Vec<_> = (0..block_size).map(|_| (0u32,)).collect();
        let c: Vec<_> = (0..block_size).map(|_| (0u32,)).collect();
        let d: Vec<_> = (0..block_size).map(|_| (0u32,)).collect();
        let e: Vec<_> = (0..block_size).map(|_| (0u32,)).collect();
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
    group.bench_with_input(BenchmarkId::new("vec-soa", 6), &6, |bencher, _| {
        let a: Vec<_> = (0..block_size).map(|_| (0u32,)).collect();
        let b: Vec<_> = (0..block_size).map(|_| (0u32,)).collect();
        let c: Vec<_> = (0..block_size).map(|_| (0u32,)).collect();
        let d: Vec<_> = (0..block_size).map(|_| (0u32,)).collect();
        let e: Vec<_> = (0..block_size).map(|_| (0u32,)).collect();
        let f: Vec<_> = (0..block_size).map(|_| (0u32,)).collect();
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
}

