extern crate specs;
#[macro_use]
extern crate specs_derive;
extern crate legion;
extern crate lazy_static;
extern crate criterion;
extern crate rand;

use suits::*;
use criterion::Criterion;
use std::time::Duration;

mod suits;
mod utils;

fn main() {
    let mut criterion = Criterion::default()
        .warm_up_time(Duration::from_millis(100))
        .configure_from_args();

    for &dataset_size in &[10, 25, 100, 250, 1_000, 2_500, 10_000] {
        let mut group = criterion.benchmark_group(format!("iteration-{}", dataset_size));
        baseline_benches::iteration_aos(&mut group, dataset_size);
        baseline_benches::iteration_soa(&mut group, dataset_size);
        specs_benches::iteration(&mut group, dataset_size);
        legion_benches::iteration(&mut group, dataset_size);
        group.finish();
    }

    for &archetype_count in &[1, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024] {
        for &dataset_size in &[1, 5, 25, 100, 250, 1000] {
            let mut group = criterion.benchmark_group(format!("iteration-archetypes-{}", archetype_count));
            legion_benches::iteration_by_archetypes(&mut group, archetype_count, dataset_size);
            specs_benches::iteration_by_archetypes(&mut group, archetype_count, dataset_size);
            group.finish();
        }
    }

    for &alt_size in &[1, 4, 16, 64, 256, 1024, 4096, 16384] {
        let mut group = criterion.benchmark_group("iteration-saturation-small-dataset");
        legion_benches::iteration_by_saturation(&mut group, 16384, alt_size, false);
        specs_benches::iteration_by_saturation(&mut group, 16384, alt_size, false);
        group.finish();
    }

    for &alt_size in &[1, 4, 16, 64, 256, 1024, 4096, 16384, 32768, 65536, 131072, 262144, 524288, 1048576] {
        let mut group = criterion.benchmark_group("iteration-saturation-large-dataset");
        legion_benches::iteration_by_saturation(&mut group, 1048576, alt_size, false);
        specs_benches::iteration_by_saturation(&mut group, 1048576, alt_size, false);
        group.finish();
    }

    for &alt_size in &[1, 4, 16, 64, 256, 1024, 4096, 16384] {
        let mut group = criterion.benchmark_group("iteration-saturation-small-dataset-reorder");
        legion_benches::iteration_by_saturation(&mut group, 16384, alt_size, true);
        specs_benches::iteration_by_saturation(&mut group, 16384, alt_size, true);
        group.finish();
    }

    for &alt_size in &[1, 4, 16, 64, 256, 1024, 4096, 16384] {
        let mut group = criterion.benchmark_group("iteration-saturation-huge-value");
        legion_benches::iteration_by_saturation_huge(&mut group, 16384, alt_size, false);
        specs_benches::iteration_by_saturation_huge(&mut group, 16384, alt_size, false);
        group.finish();
    }

    for &alt_size in &[1, 4, 16, 64, 256, 1024, 4096, 16384] {
        let mut group = criterion.benchmark_group("iteration-saturation-huge-reorder");
        legion_benches::iteration_by_saturation_huge(&mut group, 16384, alt_size, true);
        specs_benches::iteration_by_saturation_huge(&mut group, 16384, alt_size, true);
        group.finish();
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