use specs::prelude::*;
use criterion::BenchmarkGroup;
use criterion::measurement::WallTime;

#[derive(Component, Copy, Clone, Debug, Default)]
#[storage(VecStorage)]
pub struct Coordinate(f32, f32);

#[derive(Component, Copy, Clone, Debug, Default)]
#[storage(VecStorage)]
pub struct XY(u32, u32);

#[derive(Component, Copy, Clone, Debug, Default)]
#[storage(VecStorage)]
pub struct Rounds(u32);

pub struct Mandelbrot;
impl<'a>  System<'a> for Mandelbrot {
    type SystemData = (ReadStorage<'a, Coordinate>, WriteStorage<'a, Rounds>);

    fn run(&mut self, (xy, mut rounds): Self::SystemData) {
        (&xy, &mut rounds).par_join()
            .for_each(|(&Coordinate(x0, y0), rounds)| {
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

    }
}

pub fn specs_mandelbrot(group: &mut BenchmarkGroup<WallTime>, width: u32, height: u32) {
    group.bench_function("specs", |bencher| {
        let mut world = World::new();
        world.register::<Coordinate>();
        world.register::<XY>();
        world.register::<Rounds>();

        let entities: Vec<_> = world.create_iter().take((width * height) as usize).collect();
        {
            let mut xy = world.write_component::<XY>();
            let mut coordinate = world.write_component::<Coordinate>();
            let mut rounds = world.write_component::<Rounds>();
            let mut entities = entities.iter();
            for y in 0..height {
                for x in 0..width {
                    let entity = *entities.next().unwrap();
                    xy.insert(entity, XY(x, y)).unwrap();
                    let (x, y) = (x as f32 * 3.5 - 2.5, y as f32 * 2. - 1.);
                    coordinate.insert(entity, Coordinate(x, y)).unwrap();
                    rounds.insert(entity, Rounds(0)).unwrap();
                }
            }
        }

        let mut dispatch = DispatcherBuilder::new()
            .with(Mandelbrot, "mandelbrot", &[])
            .build();

        bencher.iter(|| {
            dispatch.dispatch(&mut world);
        });
    });
}