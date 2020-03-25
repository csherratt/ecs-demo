extern crate legion;

use criterion::*;
use legion::prelude::*;
use criterion::BatchSize::PerIteration;
use std::time::{Duration, Instant};
use std::cmp::{max, min};

#[derive(Copy, Clone, Debug, Default)]
struct A(u32);

#[derive(Copy, Clone, Debug, Default)]
struct B(u32);

#[derive(Copy, Clone, Debug, Default)]
struct C(u32);

#[derive(Copy, Clone, Debug, Default)]
struct D(u32);

#[derive(Copy, Clone, Debug, Default)]
struct E(u32);

#[derive(Copy, Clone, Debug, Default)]
struct F(u32);

#[derive(Copy, Clone, Debug, Default)]
struct G(u32);

#[derive(Copy, Clone, Debug, Default)]
struct H(u32);

#[derive(Copy, Clone, Debug, Default)]
struct I(u32);

fn legion_world_create() -> World {
    let world = World::new();
    world
}

pub fn legion_create(c: &mut Criterion) {
    let mut group = c.benchmark_group("legion-create");
    group.bench_with_input(BenchmarkId::new("once", 0), &0, |bencher, _| {
        bencher.iter_custom(|iters| {
            let mut world = legion_world_create();
            let start = Instant::now();
            for _ in 0..iters {
                world.insert((), Some(
                    ()
                ).into_iter());
            }
            let end = Instant::now();
            end.duration_since(start)
        });
    });
    group.bench_with_input(BenchmarkId::new("once", 1), &1, |bencher, _| {
        bencher.iter_custom(|iters| {
            let mut world = legion_world_create();
            let start = Instant::now();
            for _ in 0..iters {
                world.insert((), Some(
                    (A::default(),)
                ).into_iter());
            }
            let end = Instant::now();
            end.duration_since(start)
        });
    });
    group.bench_with_input(BenchmarkId::new("once", 2), &2, |bencher, _| {
        bencher.iter_custom(|iters| {
            let mut world = legion_world_create();
            let start = Instant::now();
            for _ in 0..iters {
                world.insert((), Some(
                    (A::default(), B::default())
                ).into_iter());
            }
            let end = Instant::now();
            end.duration_since(start)
        });
    });
    group.bench_with_input(BenchmarkId::new("once", 4), &4, |bencher, _| {
        bencher.iter_custom(|iters| {
            let mut world = legion_world_create();
            let start = Instant::now();
            for _ in 0..iters {
                world.insert((), Some(
                    (A::default(), B::default(), C::default(), D::default())
                ).into_iter());
            }
            let end = Instant::now();
            end.duration_since(start)
        });
    });
    group.bench_with_input(BenchmarkId::new("once", 6), &6, |bencher, _| {
        bencher.iter_custom(|iters| {
            let mut world = legion_world_create();
            let start = Instant::now();
            for _ in 0..iters {
                world.insert((), Some(
                    (A::default(), B::default(), C::default(), D::default(),
                     E::default(), F::default())
                ).into_iter());
            }
            let end = Instant::now();
            end.duration_since(start)
        });
    });
    group.bench_with_input(BenchmarkId::new("once", 8), &8, |bencher, _| {
        bencher.iter_custom(|iters| {
            let mut world = legion_world_create();
            let start = Instant::now();
            for _ in 0..iters {
                world.insert((), Some(
                    (A::default(), B::default(), C::default(), D::default(),
                     E::default(), F::default(), G::default(), H::default())
                ).into_iter());
            }
            let end = Instant::now();
            end.duration_since(start)
        });
    });
    group.finish();

    let mut group = c.benchmark_group("legion-create");
    group.bench_with_input(BenchmarkId::new("bulk", 0), &0, |bencher, _| {
        bencher.iter_custom(|iters| {
            let mut world = legion_world_create();
            let start = Instant::now();
            world.insert(
                (),
                (0..iters).map(|_|
                    ()
                )
            );
            let end = Instant::now();
            end.duration_since(start)
        });
    });

    group.bench_with_input(BenchmarkId::new("bulk", 1), &1, |bencher, _| {
        bencher.iter_custom(|iters| {
            let mut world = legion_world_create();
            let start = Instant::now();
            world.insert(
                (),
                (0..iters).map(|_|
                    (A::default(),)
                )
            );
            let end = Instant::now();
            end.duration_since(start)
        });
    });

    group.bench_with_input(BenchmarkId::new("bulk", 2), &2, |bencher, _| {
        bencher.iter_custom(|iters| {
            let mut world = legion_world_create();
            let start = Instant::now();
            world.insert(
                (),
                (0..iters).map(|_|
                    (A::default(), B::default())
                )
            );
            let end = Instant::now();
            end.duration_since(start)
        });
    });

    group.bench_with_input(BenchmarkId::new("bulk", 4), &4, |bencher, _| {
        bencher.iter_custom(|iters| {
            let mut world = legion_world_create();
            let start = Instant::now();
            world.insert(
                (),
                (0..iters).map(|_|
                    (A::default(), B::default(), C::default(), D::default())
                )
            );
            let end = Instant::now();
            end.duration_since(start)
        });
    });

    group.bench_with_input(BenchmarkId::new("bulk", 6), &6, |bencher, _| {
        bencher.iter_custom(|iters| {
            let mut world = legion_world_create();
            let start = Instant::now();
            world.insert(
                (),
                (0..iters).map(|_|
                    (A::default(), B::default(), C::default(), D::default(),
                     E::default(), F::default())
                )
            );
            let end = Instant::now();
            end.duration_since(start)
        });
    });

    group.bench_with_input(BenchmarkId::new("bulk", 8), &8, |bencher, _| {
        bencher.iter_custom(|iters| {
            let mut world = legion_world_create();
            let start = Instant::now();
            world.insert(
                (),
                (0..iters).map(|_|
                    (A::default(), B::default(), C::default(), D::default(),
                     E::default(), F::default(), G::default(), H::default())
                )
            );
            let end = Instant::now();
            end.duration_since(start)
        });
    });
    group.finish();
}

pub fn legion_delete(c: &mut Criterion) {
    c.bench_function("legion-delete-1-of-8", |bencher| {
        bencher.iter_custom(|iters| {
            let mut world = legion_world_create();
            let entites: Vec<_> = world.insert(
                (), (0..iters).map(|_| {
                    (A::default(), B::default(), C::default(), D::default(),
                     E::default(), F::default(), G::default(), H::default())
                })
            ).into();

            let start = Instant::now();
            for entity in entites {
                world.remove_component::<A>(entity);
            }
            let end = Instant::now();
            end.duration_since(start)
        });
    });
    c.bench_function("legion-delete-8", |bencher| {
        bencher.iter_custom(|iters| {
            let mut world = legion_world_create();
            let entites: Vec<_> = world.insert(
                (), (0..iters).map(|_| {
                    (A::default(), B::default(), C::default(), D::default(),
                     E::default(), F::default(), G::default(), H::default())
                })
            ).into();

            let start = Instant::now();
            for entity in entites {
                world.remove_component::<A>(entity);
                world.remove_component::<B>(entity);
                world.remove_component::<C>(entity);
                world.remove_component::<D>(entity);
                world.remove_component::<E>(entity);
                world.remove_component::<F>(entity);
                world.remove_component::<G>(entity);
                world.remove_component::<H>(entity);
            }
            let end = Instant::now();
            end.duration_since(start)
        });
    });
}

pub fn iteration(c: &mut Criterion) {
    let block_size = 10_000;

    let mut group = c.benchmark_group("legion");
    group.bench_with_input(BenchmarkId::new("iteration-1-archetype-0-tags", 1), &1, |bencher, _| {
        let mut world = legion_world_create();
        world.insert(
            (),
            (0..block_size).map(|_| (A(0),))
        );
        bencher.iter_custom(|iters| {
            let query = <Read<A>>::query();
            let mut count = iters as i64;
            let start = Instant::now();
            while count > 0 {
                for value in query.iter(&mut world).take(min(block_size, count as usize)) {
                    criterion::black_box(*value);
                }
                count -= block_size as i64;
            }
            Instant::now().duration_since(start)
        })
    });
    group.bench_with_input(BenchmarkId::new("iteration-1-archetype-0-tags", 2), &2, |bencher, _| {
        let mut world = legion_world_create();
        world.insert(
            (),
            (0..block_size).map(|_| (A(0), B(0)))
        );
        bencher.iter_custom(|iters| {
            let query = <(Read<A>, Read<B>)>::query();
            let mut count = iters as i64;
            let start = Instant::now();
            while count > 0 {
                for (a, b) in query.iter(&mut world).take(min(block_size, count as usize)) {
                    criterion::black_box((*a, *b));
                }
                count -= block_size as i64;
            }
            Instant::now().duration_since(start)
        })
    });
    group.bench_with_input(BenchmarkId::new("iteration-1-archetype-0-tags", 3), &3, |bencher, _| {
        let mut world = legion_world_create();
        world.insert(
            (),
            (0..block_size).map(|_| (A(0), B(0), C(0)))
        );
        bencher.iter_custom(|iters| {
            let query = <(Read<A>, Read<B>, Read<C>)>::query();
            let mut count = iters as i64;
            let start = Instant::now();
            while count > 0 {
                for (a, b, c) in query.iter(&mut world).take(min(block_size, count as usize)) {
                    criterion::black_box((*a, *b, *c));
                }
                count -= block_size as i64;
            }
            Instant::now().duration_since(start)
        })
    });
    group.bench_with_input(BenchmarkId::new("iteration-1-archetype-0-tags", 4), &4, |bencher, _| {
        let mut world = legion_world_create();
        world.insert(
            (),
            (0..block_size).map(|_| (A(0), B(0), C(0), D(0)))
        );
        bencher.iter_custom(|iters| {
            let query = <(Read<A>, Read<B>, Read<C>, Read<D>)>::query();
            let mut count = iters as i64;
            let start = Instant::now();
            while count > 0 {
                for (a, b, c, d) in query.iter(&mut world).take(min(block_size, count as usize)) {
                    criterion::black_box((*a, *b, *c, *d));
                }
                count -= block_size as i64;
            }
            Instant::now().duration_since(start)
        })
    });
    group.bench_with_input(BenchmarkId::new("iteration-1-archetype-0-tags", 5), &5, |bencher, _| {
        let mut world = legion_world_create();
        world.insert(
            (),
            (0..block_size).map(|_| (A(0), B(0), C(0), D(0), E(0)))
        );
        bencher.iter_custom(|iters| {
            let query = <(Read<A>, Read<B>, Read<C>, Read<D>, Read<E>)>::query();
            let mut count = iters as i64;
            let start = Instant::now();
            while count > 0 {
                for (a, b, c, d, e) in query.iter(&mut world).take(min(block_size, count as usize)) {
                    criterion::black_box((*a, *b, *c, *d, *e));
                }
                count -= block_size as i64;
            }
            Instant::now().duration_since(start)
        })
    });
    group.bench_with_input(BenchmarkId::new("iteration-1-archetype-0-tags", 6), &6, |bencher, _| {
        let mut world = legion_world_create();
        world.insert(
            (),
            (0..block_size).map(|_| (A(0), B(0), C(0), D(0), E(0), F(0)))
        );
        bencher.iter_custom(|iters| {
            let query = <(Read<A>, Read<B>, Read<C>, Read<D>, Read<E>, Read<F>)>::query();
            let mut count = iters as i64;
            let start = Instant::now();
            while count > 0 {
                for (a, b, c, d, e, f) in query.iter(&mut world).take(min(block_size, count as usize)) {
                    criterion::black_box((*a, *b, *c, *d, *e, *f));
                }
                count -= block_size as i64;
            }
            Instant::now().duration_since(start)
        })
    });
}

criterion_group!(
    basic,
    legion_create,
    legion_delete,
    iteration
);
criterion_main!(basic);