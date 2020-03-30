use criterion::BenchmarkGroup;
use criterion::measurement::WallTime;
use legion::prelude::*;

#[derive(Copy, Clone, Debug, Default)]
pub struct Coordinate(f32, f32);

#[derive(Copy, Clone, Debug, Default)]
pub struct XY(u32, u32);

#[derive(Copy, Clone, Debug, Default)]
pub struct Rounds(u32);

pub fn legion_mandelbrot(group: &mut BenchmarkGroup<WallTime>, width: u32, height: u32) {
    group.bench_function("legion", |bencher| {
        let mut world = World::new();
        world.insert(
            (),
            (0..width)
                .flat_map(|x| (0..height).map(move |y| (x, y)))
                .map(|(x, y)| {
                    let (xf, yf) = (x as f32 * 3.5 - 2.5, y as f32 * 2. - 1.);
                    (XY(x, y), Coordinate(xf, yf), Rounds(0))
                })
        );

        let query = <(Read<Coordinate>, Write<Rounds>)>::query();
        bencher.iter(|| {
            query.par_for_each(&mut world, |(coord, mut rounds)| {
                let Coordinate(x0, y0) = *coord;
                let (mut x, mut y) = (0., 0.);
                let mut i = 0;
                while i < 1_000 && x*x + y*y <= 4. {
                    let x_temp = x*x - y*y + x0;
                    y = 2.*x*y + y0;
                    x = x_temp;
                    i += 1;
                }
                *rounds = Rounds(i);
            });
        });
    });
}