extern crate specs;
#[macro_use]
extern crate specs_derive;
extern crate legion;
extern crate lazy_static;
extern crate criterion;

use suits::*;
use criterion::Criterion;
use std::time::Duration;

mod suits;
mod utils;

fn main() {
    let mut criterion = Criterion::default()
        .warm_up_time(Duration::from_millis(100))
        .configure_from_args();

    let mut group = criterion.benchmark_group("iteration");
    baseline_benches::iteration_aos(&mut group);
    baseline_benches::iteration_soa(&mut group);
    specs_benches::iteration(&mut group);
    legion_benches::iteration(&mut group);
    group.finish();

    let mut group = criterion.benchmark_group("iteration-archetypes");
    legion_benches::iteration_by_archetypes(&mut group);
    group.finish();

    let mut group = criterion.benchmark_group("create");
    specs_benches::specs_create(&mut group);
    legion_benches::legion_create(&mut group);
    group.finish();

    let mut group = criterion.benchmark_group("create");
    specs_benches::specs_delete(&mut group);
    legion_benches::legion_delete(&mut group);
    group.finish();

    criterion.final_summary();
}