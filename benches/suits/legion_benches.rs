use criterion::*;
use legion::prelude::*;
use criterion::measurement::WallTime;
use super::super::utils::{Warm, Cold, CustomBencher};
use std::time::Instant;
use crate::suits::BLOCK_SIZE;

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

#[derive(Copy, Clone, Debug, Default)]
struct J(u32);

#[derive(Copy, Clone, Debug, Default)]
struct K(u32);

fn legion_world_create() -> World {
    let world = World::new();
    world
}

pub fn legion_create(group: &mut BenchmarkGroup<WallTime>) {
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
}

pub fn legion_delete(group: &mut BenchmarkGroup<WallTime>) {
    group.bench_function("legion-delete-1-of-8", |bencher| {
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
    group.bench_function("legion-delete-8", |bencher| {
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

pub fn iteration(group: &mut BenchmarkGroup<WallTime>) {
    bench_with::<Warm>(&mut *group, "legion-warm");
    bench_with::<Cold>(&mut *group, "legion-cold");

    fn bench_with<BENCH>(group: &mut BenchmarkGroup<WallTime>, name: &str)
        where BENCH: CustomBencher
    {
        group.bench_with_input(BenchmarkId::new(name, 1), &1, |bencher, _| {
            let mut world = legion_world_create();
            world.insert(
                (),
                (0..BLOCK_SIZE).map(|_| (A(0),))
            );
            let query = <Read<A>>::query();
            BENCH::run(bencher, BLOCK_SIZE as u32, |iters| {
                for a in query.iter(&mut world).take(iters as usize) {
                    criterion::black_box(*a);
                }
            });
        });
        group.bench_with_input(BenchmarkId::new(name, 2), &2, |bencher, _| {
            let mut world = legion_world_create();
            world.insert(
                (),
                (0..BLOCK_SIZE).map(|_| (A(0), B(0)))
            );
            let query = <(Read<A>, Read<B>)>::query();
            BENCH::run(bencher, BLOCK_SIZE as u32, |iters| {
                for (a, b) in query.iter(&mut world).take(iters as usize) {
                    criterion::black_box((*a, *b));
                }
            });
        });
        group.bench_with_input(BenchmarkId::new(name, 3), &3, |bencher, _| {
            let mut world = legion_world_create();
            world.insert(
                (),
                (0..BLOCK_SIZE).map(|_| (A(0), B(0), C(0)))
            );
            let query = <(Read<A>, Read<B>, Read<C>)>::query();
            BENCH::run(bencher, BLOCK_SIZE as u32, |iters| {
                for (a, b, c) in query.iter(&mut world).take(iters as usize) {
                    criterion::black_box((*a, *b, *c));
                }
            });
        });
        group.bench_with_input(BenchmarkId::new(name, 4), &4, |bencher, _| {
            let mut world = legion_world_create();
            world.insert(
                (),
                (0..BLOCK_SIZE).map(|_| (A(0), B(0), C(0), D(0)))
            );
            let query = <(Read<A>, Read<B>, Read<C>, Read<D>)>::query();
            BENCH::run(bencher, BLOCK_SIZE as u32, |iters| {
                for (a, b, c, d) in query.iter(&mut world).take(iters as usize) {
                    criterion::black_box((*a, *b, *c, *d));
                }
            });
        });
        group.bench_with_input(BenchmarkId::new(name, 5), &5, |bencher, _| {
            let mut world = legion_world_create();
            world.insert(
                (),
                (0..BLOCK_SIZE).map(|_| (A(0), B(0), C(0), D(0), E(0)))
            );
            let query = <(Read<A>, Read<B>, Read<C>, Read<D>, Read<E>)>::query();
            BENCH::run(bencher, BLOCK_SIZE as u32, |iters| {
                for (a, b, c, d, e) in query.iter(&mut world).take(iters as usize) {
                    criterion::black_box((*a, *b, *c, *d, *e));
                }
            });
        });
        group.bench_with_input(BenchmarkId::new(name, 6), &6, |bencher, _| {
            let mut world = legion_world_create();
            world.insert(
                (),
                (0..BLOCK_SIZE).map(|_| (A(0), B(0), C(0), D(0), E(0), F(0)))
            );
            let query = <(Read<A>, Read<B>, Read<C>, Read<D>, Read<E>, Read<F>)>::query();
            BENCH::run(bencher, BLOCK_SIZE as u32, |iters| {
                for (a, b, c, d, e, f) in query.iter(&mut world).take(iters as usize) {
                    criterion::black_box((*a, *b, *c, *d, *e, *f));
                }
            });
        });
    }
}

pub fn iteration_by_archetypes(group: &mut BenchmarkGroup<WallTime>) {
    fn build_with_archetypes(count: u32) -> World {
        let mut world = World::new();
        for i in 0..count {
            let entity = world.insert(
                (),
                Some((A(i),)).into_iter()
            )[0];

            if count == 0 {
                continue;
            }

            let n = i % count;
            if n & 1 != 0 { world.add_component(entity, B(i)); }
            if n & 2 != 0 { world.add_component(entity, C(i)); }
            if n & 4 != 0 { world.add_component(entity, D(i)); }
            if n & 8 != 0 { world.add_component(entity, E(i)); }
            if n & 16 != 0 { world.add_component(entity, F(i)); }
            if n & 32 != 0 { world.add_component(entity, G(i)); }
            if n & 64 != 0 { world.add_component(entity, H(i)); }
            if n & 128 != 0 { world.add_component(entity, I(i)); }
            if n & 256 != 0 { world.add_component(entity, J(i)); }
            if n & 512 != 0 { world.add_component(entity, K(i)); }
        }
        world
    }

    for count in (1..=128).step_by(8) {
        group.bench_with_input(BenchmarkId::new("legion-cold", count), &count, |bencher, &count| {
            let mut world = build_with_archetypes(count);
            let query = <Read<A>>::query();
            Cold::run(bencher, count, |max| {
                for value in query.iter(&mut world).take(max as usize) {
                    criterion::black_box(*value);
                }
            });
        });
    }
    for count in (1..=128).step_by(8) {
        group.bench_with_input(BenchmarkId::new("legion-warm", count), &count, |bencher, &count| {
            let mut world = build_with_archetypes(count);
            let query = <Read<A>>::query();
            Warm::run(bencher, count, |max| {
                for value in query.iter(&mut world).take(max as usize) {
                    criterion::black_box(*value);
                }
            });
        });
    }
}
