use criterion::*;
use specs::prelude::*;
use criterion::measurement::WallTime;
use super::super::utils::{Cold, Warm, CustomBencher};
use std::time::Instant;
use crate::suits::BLOCK_SIZE;

#[derive(Component, Copy, Clone, Debug, Default)]
#[storage(VecStorage)]
struct A(u32);

#[derive(Component, Copy, Clone, Debug, Default)]
#[storage(VecStorage)]
struct B(u32);

#[derive(Component, Copy, Clone, Debug, Default)]
#[storage(VecStorage)]
struct C(u32);

#[derive(Component, Copy, Clone, Debug, Default)]
#[storage(VecStorage)]
struct D(u32);

#[derive(Component, Copy, Clone, Debug, Default)]
#[storage(VecStorage)]
struct E(u32);

#[derive(Component, Copy, Clone, Debug, Default)]
#[storage(VecStorage)]
struct F(u32);

#[derive(Component, Copy, Clone, Debug, Default)]
#[storage(VecStorage)]
struct G(u32);

#[derive(Component, Copy, Clone, Debug, Default)]
#[storage(VecStorage)]
struct H(u32);

fn specs_world_create() -> specs::World {
    let mut world = specs::World::new();
    world.register::<A>();
    world.register::<B>();
    world.register::<C>();
    world.register::<D>();
    world.register::<E>();
    world.register::<F>();
    world.register::<G>();
    world.register::<H>();
    world
}

type Args<'a> = (Entities<'a>,
                 WriteStorage<'a, A>, WriteStorage<'a, B>, WriteStorage<'a, C>, WriteStorage<'a, D>,
                 WriteStorage<'a, E>, WriteStorage<'a, F>, WriteStorage<'a, G>, WriteStorage<'a, H>);

pub fn specs_create(group: &mut BenchmarkGroup<WallTime>) {
    group.bench_with_input(BenchmarkId::new("external", 0), &0, |bencher, _| {
        bencher.iter_custom(|iters| {
            let mut world = specs_world_create();
            let start = Instant::now();
            for _ in 0..iters {
                world.create_entity()
                    .build();
            }
            let end = Instant::now();
            end.duration_since(start)
        });
    });
    group.bench_with_input(BenchmarkId::new("external", 1), &1, |bencher, _| {
        bencher.iter_custom(|iters| {
            let mut world = specs_world_create();
            let start = Instant::now();
            for _ in 0..iters {
                world.create_entity()
                    .with(A::default())
                    .build();
            }
            let end = Instant::now();
            end.duration_since(start)
        });
    });
    group.bench_with_input(BenchmarkId::new("external", 2), &2, |bencher, _| {
        bencher.iter_custom(|iters| {
            let mut world = specs_world_create();
            let start = Instant::now();
            for _ in 0..iters {
                world.create_entity()
                    .with(A::default())
                    .with(B::default())
                    .build();
            }
            let end = Instant::now();
            end.duration_since(start)
        });
    });
    group.bench_with_input(BenchmarkId::new("external", 4), &4, |bencher, _| {
        bencher.iter_custom(|iters| {
            let mut world = specs_world_create();
            let start = Instant::now();
            for _ in 0..iters {
                world.create_entity()
                    .with(A::default())
                    .with(B::default())
                    .with(C::default())
                    .with(D::default())
                    .build();
            }
            let end = Instant::now();
            end.duration_since(start)
        });
    });
    group.bench_with_input(BenchmarkId::new("external", 6), &6, |bencher, _| {
        bencher.iter_custom(|iters| {
            let mut world = specs_world_create();
            let start = Instant::now();
            for _ in 0..iters {
                world.create_entity()
                    .with(A::default())
                    .with(B::default())
                    .with(C::default())
                    .with(D::default())
                    .with(E::default())
                    .with(F::default())
                    .build();
            }
            let end = Instant::now();
            end.duration_since(start)
        });
    });
    group.bench_with_input(BenchmarkId::new("external", 8), &8, |bencher, _| {
        bencher.iter_custom(|iters| {
            let mut world = specs_world_create();
            let start = Instant::now();
            for _ in 0..iters {
                world.create_entity()
                    .with(A::default())
                    .with(B::default())
                    .with(C::default())
                    .with(D::default())
                    .with(E::default())
                    .with(F::default())
                    .with(G::default())
                    .with(H::default())
                    .build();
            }
            let end = Instant::now();
            end.duration_since(start)
        });
    });

    group.bench_with_input(BenchmarkId::new("system", 0), &0, |bencher, _| {
        bencher.iter_custom(|iters| {
            let mut world = specs_world_create();
            world.exec(|(entities, _, _, _, _, _, _, _, _): Args| {
                let start = Instant::now();
                entities.create_iter().take(iters as usize).for_each(|_| {});
                let end = Instant::now();
                end.duration_since(start)
            })
        });
    });
    group.bench_with_input(BenchmarkId::new("system", 1), &1, |bencher, _| {
        bencher.iter_custom(|iters| {
            let mut world = specs_world_create();
            world.exec(|(entities, mut a, _, _, _, _, _, _, _): Args| {
                let start = Instant::now();
                for entity in entities.create_iter().take(iters as usize) {
                    a.insert(entity, Default::default()).unwrap();
                }
                let end = Instant::now();
                end.duration_since(start)
            })
        });
    });
    group.bench_with_input(BenchmarkId::new("system", 2), &2, |bencher, _| {
        bencher.iter_custom(|iters| {
            let mut world = specs_world_create();
            world.exec(|(entities, mut a, mut b, _, _, _, _, _, _): Args| {
                let start = Instant::now();
                for entity in entities.create_iter().take(iters as usize) {
                    a.insert(entity, Default::default()).unwrap();
                    b.insert(entity, Default::default()).unwrap();
                }
                let end = Instant::now();
                end.duration_since(start)
            })
        });
    });
    group.bench_with_input(BenchmarkId::new("system", 4), &4, |bencher, _| {
        bencher.iter_custom(|iters| {
            let mut world = specs_world_create();
            world.exec(|(entities, mut a, mut b, mut c, mut d, _, _, _, _): Args| {
                let start = Instant::now();
                for entity in entities.create_iter().take(iters as usize) {
                    a.insert(entity, Default::default()).unwrap();
                    b.insert(entity, Default::default()).unwrap();
                    c.insert(entity, Default::default()).unwrap();
                    d.insert(entity, Default::default()).unwrap();
                }
                let end = Instant::now();
                end.duration_since(start)
            })
        });
    });
    group.bench_with_input(BenchmarkId::new("system", 6), &6, |bencher, _| {
        bencher.iter_custom(|iters| {
            let mut world = specs_world_create();
            world.exec(|(entities, mut a, mut b, mut c, mut d, mut e, mut f, _, _): Args| {
                let start = Instant::now();
                for entity in entities.create_iter().take(iters as usize) {
                    a.insert(entity, Default::default()).unwrap();
                    b.insert(entity, Default::default()).unwrap();
                    c.insert(entity, Default::default()).unwrap();
                    d.insert(entity, Default::default()).unwrap();
                    e.insert(entity, Default::default()).unwrap();
                    f.insert(entity, Default::default()).unwrap();
                }
                let end = Instant::now();
                end.duration_since(start)
            })
        });
    });
    group.bench_with_input(BenchmarkId::new("system", 8), &8, |bencher, _| {
        bencher.iter_custom(|iters| {
            let mut world = specs_world_create();
            world.exec(|(entities, mut a, mut b, mut c, mut d, mut e, mut f, mut g, mut h): Args| {
                let start = Instant::now();
                for entity in entities.create_iter().take(iters as usize) {
                    a.insert(entity, Default::default()).unwrap();
                    b.insert(entity, Default::default()).unwrap();
                    c.insert(entity, Default::default()).unwrap();
                    d.insert(entity, Default::default()).unwrap();
                    e.insert(entity, Default::default()).unwrap();
                    f.insert(entity, Default::default()).unwrap();
                    g.insert(entity, Default::default()).unwrap();
                    h.insert(entity, Default::default()).unwrap();
                }
                let end = Instant::now();
                end.duration_since(start)
            })
        });
    });
}

pub fn specs_delete(group: &mut BenchmarkGroup<WallTime>) {
    group.bench_function("specs-delete-1-of-8", |bencher| {
        bencher.iter_custom(|iters| {
            let mut world = specs_world_create();
            let enitites: Vec<_> = (0..iters).map(|_| {
                world.create_entity()
                    .with(A::default())
                    .with(B::default())
                    .with(C::default())
                    .with(D::default())
                    .with(E::default())
                    .with(F::default())
                    .with(G::default())
                    .with(H::default())
                    .build()
            }).collect();

            let mut a= world.system_data::<WriteStorage<A>>();
            let start = Instant::now();
            for entity in enitites {
                a.remove(entity);
            }
            let end = Instant::now();
            end.duration_since(start)
        });
    });
    group.bench_function("specs-delete-8", |bencher| {
        bencher.iter_custom(|iters| {
            let mut world = specs_world_create();
            let enitites: Vec<_> = (0..iters).map(|_| {
                world.create_entity()
                    .with(A::default())
                    .with(B::default())
                    .with(C::default())
                    .with(D::default())
                    .with(E::default())
                    .with(F::default())
                    .with(G::default())
                    .with(H::default())
                    .build()
            }).collect();

            let (mut a, mut b, mut c, mut d,
                 mut e, mut f, mut g, mut h) = world.system_data::<
                (WriteStorage<A>,
                 WriteStorage<B>,
                 WriteStorage<C>,
                 WriteStorage<D>,
                 WriteStorage<E>,
                 WriteStorage<F>,
                 WriteStorage<G>,
                 WriteStorage<H>)>();
            let start = Instant::now();
            for entity in enitites {
                a.remove(entity);
                b.remove(entity);
                c.remove(entity);
                d.remove(entity);
                e.remove(entity);
                f.remove(entity);
                g.remove(entity);
                h.remove(entity);
            }
            let end = Instant::now();
            end.duration_since(start)
        });
    });
}

fn with_world<INNER>(mut inner: INNER)
    where INNER: FnMut(ReadStorage<A>, ReadStorage<B>, ReadStorage<C>,
                       ReadStorage<D>, ReadStorage<E>, ReadStorage<F>)
{
    let mut world = specs_world_create();
    (0..BLOCK_SIZE).for_each(|_| {
        world.create_entity()
            .with(A::default())
            .with(B::default())
            .with(C::default())
            .with(D::default())
            .with(E::default())
            .with(F::default())
            .build();
    });
    let (a, b, c, d, e, f) = world.system_data::<
        (ReadStorage<A>,
         ReadStorage<B>,
         ReadStorage<C>,
         ReadStorage<D>,
         ReadStorage<E>,
         ReadStorage<F>)>();
    inner(a, b, c, d, e, f);
}

pub fn iteration(group: &mut BenchmarkGroup<WallTime>) {
    bench_with::<Cold>(&mut *group, "specs-cold");
    bench_with::<Warm>(&mut *group, "specs-warm");

    fn bench_with<BENCH>(group: &mut BenchmarkGroup<WallTime>, name: &str)
        where BENCH: CustomBencher
    {
        group.bench_with_input(BenchmarkId::new(name, 1), &1, |bencher, _| {
            with_world(|a, _, _, _, _, _| {
                BENCH::run(bencher, BLOCK_SIZE, |iters| {
                    for (a, ) in (&a, ).join().cycle().take(iters as usize) {
                        black_box(*a);
                    }
                })
            });
        });
        group.bench_with_input(BenchmarkId::new(name, 2), &2, |bencher, _| {
            with_world(|a, b, _, _, _, _| {
                BENCH::run(bencher, BLOCK_SIZE, |iters| {
                    for (a, b) in (&a, &b).join().cycle().take(iters as usize) {
                        black_box((*a, *b));
                    }
                })
            });
        });
        group.bench_with_input(BenchmarkId::new(name, 3), &3, |bencher, _| {
            with_world(|a, b, c, _, _, _| {
                BENCH::run(bencher, BLOCK_SIZE, |iters| {
                    for (a, b, c) in (&a, &b, &c).join().cycle().take(iters as usize) {
                        black_box((*a, *b, *c));
                    }
                })
            });
        });
        group.bench_with_input(BenchmarkId::new(name, 4), &4, |bencher, _| {
            with_world(|a, b, c, d, _, _| {
                BENCH::run(bencher, BLOCK_SIZE, |iters| {
                    for (a, b, c, d) in (&a, &b, &c, &d).join().cycle().take(iters as usize) {
                        black_box((*a, *b, *c, *d));
                    }
                })
            });
        });
        group.bench_with_input(BenchmarkId::new(name, 5), &5, |bencher, _| {
            with_world(|a, b, c, d, e, _| {
                BENCH::run(bencher, BLOCK_SIZE, |iters| {
                    for (a, b, c, d, e) in (&a, &b, &c, &d, &e).join().cycle().take(iters as usize) {
                        black_box((*a, *b, *c, *d, *e));
                    }
                })
            });
        });
        group.bench_with_input(BenchmarkId::new(name, 6), &6, |bencher, _| {
            with_world(|a, b, c, d, e, f| {
                BENCH::run(bencher, BLOCK_SIZE, |iters| {
                    for (a, b, c, d, e, f) in (&a, &b, &c, &d, &e, &f).join().cycle().take(iters as usize) {
                        black_box((*a, *b, *c, *d, *e, *f));
                    }
                })
            });
        });
    }
}



