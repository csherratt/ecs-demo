use criterion::*;
use criterion::measurement::WallTime;
use crate::utils::{Cold, Warm, CustomBencher};

pub fn iteration_aos(group: &mut BenchmarkGroup<WallTime>, dataset_size: usize) {
    bench_with::<Cold>(&mut *group, "vec-aos-cold", dataset_size as u32);
    bench_with::<Warm>(&mut *group, "vec-aos-warm", dataset_size as u32);

    fn bench_with<BENCH>(group: &mut BenchmarkGroup<WallTime>, name: &str, dataset_size: u32)
        where BENCH: CustomBencher
    {
        group.bench_with_input(BenchmarkId::new(name, 1), &1, |bencher, _| {
            let vec: Vec<_> = (0..dataset_size).map(|_| (0u32, )).collect();
            BENCH::run(bencher, dataset_size, |iters| {
                vec.iter().take(iters as usize).for_each(|inner| { black_box(*inner); });
            });
        });
        group.bench_with_input(BenchmarkId::new(name, 2), &2, |bencher, _| {
            let vec: Vec<_> = (0..dataset_size).map(|_| (0u32, 0u32)).collect();
            BENCH::run(bencher, dataset_size, |iters| {
                vec.iter().take(iters as usize).for_each(|inner| { black_box(*inner); });
            })
        });
        group.bench_with_input(BenchmarkId::new(name, 3), &3, |bencher, _| {
            let vec: Vec<_> = (0..dataset_size).map(|_| (0u32, 0u32, 0u32)).collect();
            BENCH::run(bencher, dataset_size, |iters| {
                vec.iter().take(iters as usize).for_each(|inner| { black_box(*inner); });
            })
        });
        group.bench_with_input(BenchmarkId::new(name, 4), &4, |bencher, _| {
            let vec: Vec<_> = (0..dataset_size).map(|_| (0u32, 0u32, 0u32, 0u32)).collect();
            BENCH::run(bencher, dataset_size, |iters| {
                vec.iter().take(iters as usize).for_each(|inner| { black_box(*inner); });
            })
        });
        group.bench_with_input(BenchmarkId::new(name, 5), &5, |bencher, _| {
            let vec: Vec<_> = (0..dataset_size).map(|_| (0u32, 0u32, 0u32, 0u32, 0u32)).collect();
            BENCH::run(bencher, dataset_size, |iters| {
                vec.iter().take(iters as usize).for_each(|inner| { black_box(*inner); });
            })
        });
        group.bench_with_input(BenchmarkId::new(name, 6), &6, |bencher, _| {
            let vec: Vec<_> = (0..dataset_size).map(|_| (0u32, 0u32, 0u32, 0u32, 0u32, 0u32)).collect();
            BENCH::run(bencher, dataset_size, |iters| {
                vec.iter().take(iters as usize).for_each(|inner| { black_box(*inner); });
            })
        });
    }
}

pub fn iteration_soa(group: &mut BenchmarkGroup<WallTime>, dataset_size: usize) {
    bench_with::<Cold>(&mut *group, "vec-soa-cold", dataset_size as u32);
    bench_with::<Warm>(&mut *group, "vec-soa-warm", dataset_size as u32);

    fn bench_with<BENCH>(group: &mut BenchmarkGroup<WallTime>, name: &str, dataset_size: u32)
        where BENCH: CustomBencher
    {
        group.bench_with_input(BenchmarkId::new(name, 1), &1, |bencher, _| {
            let a: Vec<_> = (0..dataset_size).map(|_| (0u32, )).collect();
            BENCH::run(bencher, dataset_size, |iters| {
                a.iter().take(iters as usize).for_each(|inner| { black_box(*inner); });
            })
        });
        group.bench_with_input(BenchmarkId::new(name, 2), &2, |bencher, _| {
            let a: Vec<_> = (0..dataset_size).map(|_| (0u32, )).collect();
            let b: Vec<_> = (0..dataset_size).map(|_| (0u32, )).collect();
            BENCH::run(bencher, dataset_size, |iters| {
                a.iter().map(|x| black_box(*x))
                    .zip(b.iter().map(|x| black_box(*x)))
                    .take(iters as usize)
                    .for_each(|_| ());
            })
        });
        group.bench_with_input(BenchmarkId::new(name, 3), &3, |bencher, _| {
            let a: Vec<_> = (0..dataset_size).map(|_| (0u32, )).collect();
            let b: Vec<_> = (0..dataset_size).map(|_| (0u32, )).collect();
            let c: Vec<_> = (0..dataset_size).map(|_| (0u32, )).collect();
            BENCH::run(bencher, dataset_size, |iters| {
                a.iter().map(|x| black_box(*x))
                    .zip(b.iter().map(|x| black_box(*x)))
                    .zip(c.iter().map(|x| black_box(*x)))
                    .take(iters as usize)
                    .for_each(|_| ());
            })
        });
        group.bench_with_input(BenchmarkId::new(name, 4), &4, |bencher, _| {
            let a: Vec<_> = (0..dataset_size).map(|_| (0u32, )).collect();
            let b: Vec<_> = (0..dataset_size).map(|_| (0u32, )).collect();
            let c: Vec<_> = (0..dataset_size).map(|_| (0u32, )).collect();
            let d: Vec<_> = (0..dataset_size).map(|_| (0u32, )).collect();
            BENCH::run(bencher, dataset_size, |iters| {
                a.iter().map(|x| black_box(*x))
                    .zip(b.iter().map(|x| black_box(*x)))
                    .zip(c.iter().map(|x| black_box(*x)))
                    .zip(d.iter().map(|x| black_box(*x)))
                    .take(iters as usize)
                    .for_each(|_| ());
            })
        });
        group.bench_with_input(BenchmarkId::new(name, 5), &5, |bencher, _| {
            let a: Vec<_> = (0..dataset_size).map(|_| (0u32, )).collect();
            let b: Vec<_> = (0..dataset_size).map(|_| (0u32, )).collect();
            let c: Vec<_> = (0..dataset_size).map(|_| (0u32, )).collect();
            let d: Vec<_> = (0..dataset_size).map(|_| (0u32, )).collect();
            let e: Vec<_> = (0..dataset_size).map(|_| (0u32, )).collect();
            BENCH::run(bencher, dataset_size, |iters| {
                a.iter().map(|x| black_box(*x))
                    .zip(b.iter().map(|x| black_box(*x)))
                    .zip(c.iter().map(|x| black_box(*x)))
                    .zip(d.iter().map(|x| black_box(*x)))
                    .zip(e.iter().map(|x| black_box(*x)))
                    .take(iters as usize)
                    .for_each(|_| ());
            })
        });
        group.bench_with_input(BenchmarkId::new(name, 6), &6, |bencher, _| {
            let a: Vec<_> = (0..dataset_size).map(|_| (0u32, )).collect();
            let b: Vec<_> = (0..dataset_size).map(|_| (0u32, )).collect();
            let c: Vec<_> = (0..dataset_size).map(|_| (0u32, )).collect();
            let d: Vec<_> = (0..dataset_size).map(|_| (0u32, )).collect();
            let e: Vec<_> = (0..dataset_size).map(|_| (0u32, )).collect();
            let f: Vec<_> = (0..dataset_size).map(|_| (0u32, )).collect();
            BENCH::run(bencher, dataset_size, |iters| {
                a.iter().map(|x| black_box(*x))
                    .zip(b.iter().map(|x| black_box(*x)))
                    .zip(c.iter().map(|x| black_box(*x)))
                    .zip(d.iter().map(|x| black_box(*x)))
                    .zip(e.iter().map(|x| black_box(*x)))
                    .zip(f.iter().map(|x| black_box(*x)))
                    .take(iters as usize)
                    .for_each(|_| ());
            })
        });
    }
}

