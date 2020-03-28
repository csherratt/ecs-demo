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

    for &dataset_size in &[25, 100, 250, 1000, 2500] {
        let mut group = criterion.benchmark_group(format!("iteration-{}", dataset_size));
        baseline_benches::iteration_aos(&mut group, dataset_size);
        baseline_benches::iteration_soa(&mut group, dataset_size);
        specs_benches::iteration(&mut group, dataset_size);
        legion_benches::iteration(&mut group, dataset_size);
        group.finish();
    }

    for &archetype_count in &[1, 2, 4, 8, 16, 32, 64, 128, 256] {
        for &dataset_size in &[1, 5, 25, 100, 250, 1000] {
            let mut group = criterion.benchmark_group(format!("iteration-archetypes-{}", archetype_count));
            legion_benches::iteration_by_archetypes(&mut group, archetype_count, dataset_size);
            specs_benches::iteration_by_archetypes(&mut group, archetype_count, dataset_size);
            group.finish();
        }
    }

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