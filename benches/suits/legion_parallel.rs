use criterion::{BenchmarkGroup, Bencher};
use criterion::measurement::WallTime;
use legion::prelude::*;

#[derive(Copy, Clone, Debug, Default)]
pub struct Coordinate(f32, f32);

#[derive(Copy, Clone, Debug, Default)]
pub struct XY(u32, u32);

#[derive(Copy, Clone, Debug, Default)]
pub struct Rounds(u32);

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct ShardA;
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct ShardB;
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct ShardC;
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct ShardD;
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct ShardE;
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct ShardF;
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct ShardG;
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct ShardH;

pub fn legion_mandelbrot(group: &mut BenchmarkGroup<WallTime>, width: u32, height: u32) {
    fn run_mandelbrot(bencher: &mut Bencher<WallTime>, mut world: World) {
        let query = <(Read<Coordinate>, Write<Rounds>)>::query();
        bencher.iter(|| {
            query.par_for_each(&mut world, |(coord, mut rounds)| {
                let Coordinate(x0, y0) = *coord;
                let (mut x, mut y) = (0., 0.);
                let mut i = 0;
                while i < 100 && x*x + y*y <= 4. {
                    let x_temp = x*x - y*y + x0;
                    y = 2.*x*y + y0;
                    x = x_temp;
                    i += 1;
                }
                *rounds = Rounds(i);
            });
        });
    }

    group.bench_function("legion", |bencher| {
        let mut world = World::new();
        world.insert(
            (),
            (0..width)
                .flat_map(|x| (0..height).map(move |y| (x, y)))
                .map(|(x, y)| {
                    let (xf, yf) = (
                        x as f32 / width as f32 * 3.5 - 2.5,
                        y as f32 / height as f32 * 2. - 1.
                    );
                    (XY(x, y), Coordinate(xf, yf), Rounds(0))
                })
        );
        run_mandelbrot(bencher, world);
    });


    group.bench_function("legions-sharded", |bencher| {
        let mut world = World::new();
        let entities: Vec<_> = world.insert(
            (),
            (0..width)
                .flat_map(|x| (0..height).map(move |y| (x, y)))
                .map(|(x, y)| {
                    let (xf, yf) = (
                        x as f32 / width as f32 * 3.5 - 2.5,
                        y as f32 / height as f32 * 2. - 1.
                    );
                    (XY(x, y), Coordinate(xf, yf), Rounds(0))
                })
        ).into();
        for (i, entity) in entities.into_iter().enumerate() {
            let n = i % 256;
            if n & 1 != 0 { world.add_component(entity, ShardA); }
            if n & 2 != 0 { world.add_component(entity, ShardB); }
            if n & 4 != 0 { world.add_component(entity, ShardC); }
            if n & 8 != 0 { world.add_component(entity, ShardD); }
            if n & 16 != 0 { world.add_component(entity, ShardE); }
            if n & 32 != 0 { world.add_component(entity, ShardF); }
            if n & 64 != 0 { world.add_component(entity, ShardG); }
            if n & 128 != 0 { world.add_component(entity, ShardH); }
        }
        run_mandelbrot(bencher, world);
    });

}