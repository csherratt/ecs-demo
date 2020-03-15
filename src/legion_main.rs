use legion::prelude::*;
use std::sync::mpsc::channel;
use std::sync::Mutex;

#[derive(Clone, Copy, Debug, PartialEq)]
struct Number(i64);

#[derive(Clone, Copy, Debug, PartialEq)]
struct IsPrime;

#[derive(Clone, Copy, Debug, PartialEq)]
struct Searched;


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

pub fn legion_main() {
    let universe = Universe::new();
    let mut world = universe.create_world();

    let mut start = 2;
    let max = 1_000_000;
    while start < max {
        let next = std::cmp::min(std::cmp::min(start * 2,start + 1024), max);
        //println!("start: {}, max: {}", start, max);
        world.insert((), (start..next).map(|i| (Number(i),)));
        let mut to_search = <Read<Number>>::query().filter(!tag::<Searched>());
        let (new_primes, res_new_primes) = channel();
        let (searched, res_searched) = channel();
        let new_primes = Mutex::new(new_primes);
        let searched = Mutex::new(searched);
        to_search.par_entities_for_each_immutable(&world, |upper| {
            let mut new_primes = new_primes.lock().map(|lock| lock.clone()).unwrap();
            let mut searched = searched.lock().map(|lock| lock.clone()).unwrap();
            let mut primes = <Read<Number>>::query().filter(tag::<IsPrime>());
            let is_prime = primes.iter_entities_immutable(&world)
                .map(|num| { searched.send(upper.0).unwrap(); num })
                .find(|num| (upper.1).0 > (num.1).0 && (upper.1).0 % (num.1).0 == 0).is_none();
            if is_prime {
                new_primes.send(upper.0);
            }
        });
        drop((new_primes, searched));
        for prime in res_new_primes {
            world.add_tag(prime, IsPrime);
        }
        for searched in res_searched {
            world.add_tag(searched, Searched);
        }
        start = next;
    }

    let mut query = <Read<Number>>::query().filter(tag::<IsPrime>());
    for num in query.iter(&mut world) {
        println!("{}", num.0);
    }
}