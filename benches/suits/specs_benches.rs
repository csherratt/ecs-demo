use specs::prelude::*;
use criterion::*;
use criterion::measurement::WallTime;
use super::super::utils::{Cold, Warm, CustomBencher};
use std::time::Instant;
use crate::suits::{A, B, C, D, E, F, G, H, I, J, K};

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
    world.register::<I>();
    world.register::<J>();
    world.register::<K>();
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

fn wrap_world<INNER>(world: World, mut inner: INNER)
    where INNER: FnMut(ReadStorage<A>, ReadStorage<B>, ReadStorage<C>,
                       ReadStorage<D>, ReadStorage<E>, ReadStorage<F>)
{
    let (a, b, c, d, e, f) = world.system_data::<
        (ReadStorage<A>,
         ReadStorage<B>,
         ReadStorage<C>,
         ReadStorage<D>,
         ReadStorage<E>,
         ReadStorage<F>)>();
    inner(a, b, c, d, e, f);
}

fn with_world<INNER>(dataset_size: u32,  inner: INNER)
    where INNER: FnMut(ReadStorage<A>, ReadStorage<B>, ReadStorage<C>,
                       ReadStorage<D>, ReadStorage<E>, ReadStorage<F>)
{
    let mut world = specs_world_create();
    (0..dataset_size).for_each(|_| {
        world.create_entity()
            .with(A::default())
            .with(B::default())
            .with(C::default())
            .with(D::default())
            .with(E::default())
            .with(F::default())
            .build();
    });
    wrap_world(world, inner)
}

pub fn iteration(group: &mut BenchmarkGroup<WallTime>, dataset_size: usize) {
    bench_with::<Cold>(&mut *group, "specs-cold", dataset_size as u32);
    bench_with::<Warm>(&mut *group, "specs-warm", dataset_size as u32);

    fn bench_with<BENCH>(group: &mut BenchmarkGroup<WallTime>, name: &str, dataset_size: u32)
        where BENCH: CustomBencher
    {
        group.bench_with_input(BenchmarkId::new(name, 1), &1, |bencher, _| {
            with_world(dataset_size, |a, _, _, _, _, _| {
                BENCH::run(bencher, dataset_size, |iters| {
                    for (a, ) in (&a, ).join().take(iters as usize) {
                        black_box(*a);
                    }
                })
            });
        });
        group.bench_with_input(BenchmarkId::new(name, 2), &2, |bencher, _| {
            with_world(dataset_size, |a, b, _, _, _, _| {
                BENCH::run(bencher, dataset_size, |iters| {
                    for (a, b) in (&a, &b).join().take(iters as usize) {
                        black_box((*a, *b));
                    }
                })
            });
        });
        group.bench_with_input(BenchmarkId::new(name, 3), &3, |bencher, _| {
            with_world(dataset_size, |a, b, c, _, _, _| {
                BENCH::run(bencher, dataset_size, |iters| {
                    for (a, b, c) in (&a, &b, &c).join().take(iters as usize) {
                        black_box((*a, *b, *c));
                    }
                })
            });
        });
        group.bench_with_input(BenchmarkId::new(name, 4), &4, |bencher, _| {
            with_world(dataset_size, |a, b, c, d, _, _| {
                BENCH::run(bencher, dataset_size, |iters| {
                    for (a, b, c, d) in (&a, &b, &c, &d).join().take(iters as usize) {
                        black_box((*a, *b, *c, *d));
                    }
                })
            });
        });
        group.bench_with_input(BenchmarkId::new(name, 5), &5, |bencher, _| {
            with_world(dataset_size, |a, b, c, d, e, _| {
                BENCH::run(bencher, dataset_size, |iters| {
                    for (a, b, c, d, e) in (&a, &b, &c, &d, &e).join().take(iters as usize) {
                        black_box((*a, *b, *c, *d, *e));
                    }
                })
            });
        });
        group.bench_with_input(BenchmarkId::new(name, 6), &6, |bencher, _| {
            with_world(dataset_size, |a, b, c, d, e, f| {
                BENCH::run(bencher, dataset_size, |iters| {
                    for (a, b, c, d, e, f) in (&a, &b, &c, &d, &e, &f).join().take(iters as usize) {
                        black_box((*a, *b, *c, *d, *e, *f));
                    }
                })
            });
        });
    }
}

pub fn iteration_by_archetypes(group: &mut BenchmarkGroup<WallTime>, per_archtype: usize, dataset_size: usize) {
    fn build_with_archetypes(per_archtype: usize, dataset_size: usize) -> World {
        let mut world = specs_world_create();
        for i in 0..(per_archtype*dataset_size) {
            let i = i as u32;
            let mut builder = world.create_entity()
                .with(A(i));
            if per_archtype == 1 {
                builder.build();
                continue;
            }
            let n = i % (per_archtype - 1) as u32;
            if n & 1 != 0 { builder = builder.with(B(i)); }
            if n & 2 != 0 { builder = builder.with(C(i)); }
            if n & 4 != 0 { builder = builder.with(D(i)); }
            if n & 8 != 0 { builder = builder.with(E(i)); }
            if n & 16 != 0 { builder = builder.with(F(i)); }
            if n & 32 != 0 { builder = builder.with(G(i)); }
            if n & 64 != 0 { builder = builder.with(H(i)); }
            if n & 128 != 0 { builder = builder.with(I(i)); }
            if n & 256 != 0 { builder = builder.with(J(i)); }
            if n & 512 != 0 { builder = builder.with(K(i)); }
            builder.build();
        }
        world
    }

    bench_with::<Warm>(group, "specs-warm", per_archtype, dataset_size);
    bench_with::<Cold>(group, "specs-cold", per_archtype, dataset_size);

    fn bench_with<BENCH>(group: &mut BenchmarkGroup<WallTime>, name: &str, per_archtype: usize, dataset_size: usize)
        where BENCH: CustomBencher
    {
        group.bench_with_input(BenchmarkId::new(name, dataset_size), &dataset_size, |bencher, &_| {
            let world = build_with_archetypes(per_archtype, dataset_size);
            wrap_world(world, |a, _, _, _, _, _| {
                BENCH::run(bencher, (dataset_size * per_archtype) as u32, |iters| {
                    for (a, ) in (&a, ).join().take(iters as usize) {
                        black_box(*a);
                    }
                })
            });
        });
    }
}




