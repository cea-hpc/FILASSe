use criterion::*;
use std::collections::HashMap;
use std::collections::LinkedList;
use std::collections::VecDeque;
use std::vec::Vec;

fn rng_next(s: u64) -> u64 {
    6364136223846793005 * s + 1
}
pub fn vec_deque(size: usize) {
    let v: VecDeque<usize> = (0..1024).collect();
    let mut v = v.clone();
    for _ in 0..size {
        v.pop_front();
    }
}

pub fn swap_remove(size: usize) {
    let mut rng = 43;
    let v: Vec<usize> = (0..size).collect();
    let mut v = v.clone();
    for _ in 0..512 {
        rng = rng_next(rng);
        let idx = (rng >> 32) % 512;
        v.swap_remove(idx as usize);
    }
}

pub fn vec_deque_swap_remove(size: usize) {
    let v: VecDeque<usize> = (0..size).collect();
    let mut v = v.clone();
    for _ in 0..512 {
        v.swap_remove_front(0);
    }
}

pub fn vec_remove(size: usize) {
    let v: Vec<usize> = (0..size).collect();
    let mut v = v.clone();
    for _ in 0..512 {
        v.remove(0);
    }
}

pub fn hashmap(size: usize) {
    let mut rng = 43;
    let mut h: HashMap<usize, usize> = HashMap::new();
    for i in 0..size {
        h.insert(i, i);
    }
    let mut h = h.clone();
    for _ in 0..512 {
        rng = rng_next(rng);
        let idx = (rng >> 32) % 512;
        let index = idx as usize;
        h.remove(&index);
    }
}

pub fn linked_list(size: usize) {
    let v: LinkedList<usize> = (0..size).collect();
    let mut v = v.clone();
    for _ in 0..512 {
        v.pop_front();
    }
}

pub fn comparing_bests(c: &mut Criterion) {
    let mut group = c.benchmark_group("Vec comparaison");
    let plot_config = PlotConfiguration::default().summary_scale(AxisScale::Logarithmic);
    group.plot_config(plot_config);
    for i in [1024, 10024].iter() {
        group.bench_with_input(BenchmarkId::new("deque", i), i, |b, i| {
            b.iter(|| vec_deque(*i))
        });
        group.bench_with_input(BenchmarkId::new("vec", i), i, |b, i| {
            b.iter(|| vec_remove(*i))
        });
        group.bench_with_input(BenchmarkId::new("swap_remove", i), i, |b, i| {
            b.iter(|| swap_remove(*i))
        });
        group.bench_with_input(BenchmarkId::new("Hashmap", i), i, |b, i| {
            b.iter(|| hashmap(*i))
        });
        group.bench_with_input(BenchmarkId::new("linked list", i), i, |b, i| {
            b.iter(|| linked_list(*i))
        });
    }
    group.finish();
}

criterion_group!(benches, comparing_bests);
criterion_main!(benches);
