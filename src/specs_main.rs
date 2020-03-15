use specs::prelude::*;

#[derive(Debug, Copy, Clone)]
struct Number(i64);
impl Component for Number {
    type Storage = VecStorage<Self>;
}

#[derive(Debug, Default)]
struct IsPrime;
impl Component for IsPrime {
    type Storage = NullStorage<Self>;
}

#[derive(Debug, Default)]
struct Searched;
impl Component for Searched {
    type Storage = NullStorage<Self>;
}


struct PrimeFinder {
    prime_buffer: Vec<Entity>,
    searched_buffer: Vec<Entity>
}

impl PrimeFinder {
    fn new() -> PrimeFinder {
        PrimeFinder {
            prime_buffer: Vec::new(),
            searched_buffer: Vec::new()
        }
    }
}

impl <'a> System<'a> for PrimeFinder {
    type SystemData = (Entities<'a>,  ReadStorage<'a, Number>, WriteStorage<'a, IsPrime>, WriteStorage<'a, Searched>);

    fn run(&mut self, (enitites, number, mut is_prime, mut searched): Self::SystemData) {
        let prime_buffer: Vec<_> = (&enitites, &number, !&searched).par_join()
            .filter(|(eid, &Number(to_check), _)| {
                (&number, &is_prime).par_join().find_any(
                    |(&Number(lower), _)| to_check > lower && (to_check % lower) == 0
                ).is_none()
            })
            .map(|(eid, _, _)| eid)
            .collect();
        let searched_buffer: Vec<_> = (&enitites, &number, !&searched).join().map(|(eid, _, _)| eid).collect();

        for &new_prime in &prime_buffer {
            is_prime.insert(new_prime, IsPrime);
        }
        for &searched_buffer in &searched_buffer {
            searched.insert(searched_buffer, Searched);
        }
    }
}

struct AddEntities(i64, i64);
impl <'a>  System<'a> for AddEntities {
    type SystemData = (Entities<'a>, WriteStorage<'a, Number>);

    fn run(&mut self, (enitites, mut numbers): Self::SystemData) {

        let next = std::cmp::min(std::cmp::min(self.0 * 2,self.0 + 1024), self.1);
        for i in self.0..next {
            enitites.build_entity().with(Number(i), &mut numbers).build();
        }
        self.0 = next;
    }
}

struct PrintSystem;

impl<'a> System<'a> for PrintSystem {
    type SystemData = (ReadStorage<'a, Number>, ReadStorage<'a, IsPrime>);

    fn run(&mut self, (numbers, primes): Self::SystemData) {
        for (numbers, _) in (&numbers, &primes).join() {
            println!("{}", numbers.0);
        }
    }
}

pub fn specs_main() {
    let mut world = World::new();
    world.register::<Number>();
    world.register::<IsPrime>();
    world.register::<Searched>();

    let mut dispatcher = DispatcherBuilder::new()
        .with(AddEntities(2, 1_000_000), "add", &[])
        .with(PrimeFinder::new(), "prime", &["add"])
        .build();

    for _ in 0..10_000 {
        dispatcher.dispatch_par(&mut world);
    }

    let mut prime_dispatch = DispatcherBuilder::new()
        .with(PrintSystem, "print-system", &[])
        .build();

    prime_dispatch.dispatch(&mut world);

}