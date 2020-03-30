use criterion::Bencher;
use std::time::{Duration, Instant};
use std::cmp::min;
use std::sync::Mutex;
use lazy_static::lazy_static;

fn force_cache_invalid() {
    lazy_static! {
        static ref ARRAY: Mutex<Vec<u32>> = Mutex::new((0..10_000_000).collect());
    };
    let mut guard = ARRAY.lock().unwrap();
    for item in &mut *guard {
        *item += 1;
    }
}

pub trait CustomBencher {
    fn run<F>(bencher: &mut Bencher, count: u32, inner: F)
        where F: FnMut(u32);
}

pub struct Cold;
impl CustomBencher for Cold {
    fn run<F>(bencher: &mut Bencher, count: u32, mut inner: F)
        where F: FnMut(u32)
    {
        bencher.iter_custom(|iters| {
            let mut iters = iters as i64;
            let mut duration = Duration::from_secs(0);
            while iters > 0 {
                force_cache_invalid();
                let start = Instant::now();
                inner(min(count, iters as u32));
                duration += Instant::now().duration_since(start);
                iters -= min(count as i64, iters);
            }
            duration
        });
    }
}

pub struct Warm;
impl CustomBencher for Warm {
    fn run<F>(bencher: &mut Bencher, count: u32, mut inner: F)
        where F: FnMut(u32)
    {
        bencher.iter_custom(|iters| {
            let mut iters = iters as i64;
            let mut duration = Duration::from_secs(0);
            while iters > 0 {
                let start = Instant::now();
                inner(min(count, iters as u32));
                duration += Instant::now().duration_since(start);
                iters -= min(count as i64, iters);
            }
            duration
        });
    }
}

pub fn bencher_max_size<T, F, U>(bencher: &mut Bencher, max_amount: u64, mut setup: F, mut inner: U)
    where F: FnMut() -> T,  U: FnMut(T, u64)
{
    bencher.iter_custom(|mut iters| {
        let mut total = Duration::from_nanos(0);
        while iters > 0 {
            let rounds = min(max_amount, iters);
            let value = setup();
            let start = Instant::now();
            inner(value, rounds);
            total += Instant::now().duration_since(start);
            iters -= rounds;
        }
        total
    });
}