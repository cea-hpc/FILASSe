#![feature(test)]
extern crate test;
use test::Bencher;

// Tiny LCG RNG.
fn rng_next(s: u64) -> u64 {
    6364136223846793005 * s + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[bench]
    fn bench_1000_overhead(b: &mut Bencher) {
        let mut rng = 43;
        let v: Vec<usize> = (0..1024).collect();
        b.iter(|| {
            let v = v.clone();
            for _ in 0..512 {
                rng = rng_next(rng);
                let idx = (rng >> 32) % 512;
                test::black_box(idx as usize);
            }
            v
        })
    }

    #[bench]
    fn bench_vec_1000_swap_remove(b: &mut Bencher) {
        let mut rng = 43;
        let v: Vec<usize> = (0..1024).collect();
        b.iter(|| {
            let mut v = v.clone();
            for _ in 0..512 {
                rng = rng_next(rng);
                let idx = (rng >> 32) % 512;
                test::black_box(v.swap_remove(idx as usize));
            }
            v
        })
    }

    use std::collections::VecDeque;
    #[bench]
    fn bench_vec_deque(b: &mut Bencher) {
        let v: VecDeque<usize> = (0..1024).collect();
        b.iter(|| {
            let mut v = v.clone();
            for _ in 0..512 {
                test::black_box(v.pop_front());
            }
        })
    }
    #[bench]
    fn bench_vec_deque_swao_remove(b: &mut Bencher) {
        let v: VecDeque<usize> = (0..1024).collect();
        b.iter(|| {
            let mut v = v.clone();
            for _ in 0..512 {
                test::black_box(v.swap_remove_front(0));
            }
        })
    }
    #[bench]
    fn bench_vec(b: &mut Bencher) {
        let v: Vec<usize> = (0..1024).collect();
        b.iter(|| {
            let mut v = v.clone();
            for _ in 0..512 {
                test::black_box(v.remove(0));
            }
        })
    }

    use std::collections::HashMap;
    #[bench]
    fn bench_hashmap(b: &mut Bencher) {
        let mut rng = 43;
        let mut h: HashMap<usize, usize> = HashMap::new();
        for i in 0..1024 {
            h.insert(i, i);
        }
        b.iter(|| {
            let mut h = h.clone();
            for _ in 0..512 {
                rng = rng_next(rng);
                let idx = (rng >> 32) % 512;
                let index = idx as usize;
                test::black_box(h.remove(&index));
            }
        })
    }

    use std::collections::LinkedList;
    #[bench]
    fn bench_linked_list(b: &mut Bencher) {
        let v: LinkedList<usize> = (0..1024).collect();
        b.iter(|| {
            let mut v = v.clone();
            for _ in 0..512 {
                test::black_box(v.pop_front());
            }
        })
    }
}
