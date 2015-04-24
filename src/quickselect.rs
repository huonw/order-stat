#![allow(dead_code)]

// this appears to be slower than floyd-rivest (cf. [1]).
//
// [1]: Kiwiel, K. C. (2005). "On Floyd and Rivest's SELECT
// algorithm". Theoretical Computer Science 347:
// 214–238. doi:10.1016/j.tcs.2005.06.032.

pub fn select<T: Ord>(x: &mut [T], k: usize) {
    if x.len() < 5 {
        for i in 0..x.len() {
            for j in (i+1)..x.len() {
                if x[j] < x[i] {
                    x.swap(i, j)
                }
            }
        }
        return
    }

    let pivot = partition(x);
    if k < pivot {
        select(&mut x[..pivot], k);
    } else if k > pivot {
        select(&mut x[pivot + 1..], k - pivot - 1)
    }
}

fn partition<T: Ord>(x: &mut [T]) -> usize {
    let l = x.len();
    let mut store = 0;
    {
        let (y, elem) = x.split_at_mut(l - 1);
        let elem = &mut elem[0];

        for load in 0..l - 1 {
            if y[load] < *elem {
                y.swap(load, store);
                store += 1
            }
        }
    }
    x.swap(store, l - 1);
    store
}

#[cfg(test)]
mod tests {
    use super::select;
    use quickcheck::{self, TestResult};
    use rand::{XorShiftRng, Rng};

    #[test]
    fn qc() {
        fn run(mut x: Vec<i32>, k: usize) -> TestResult {
            if k >= x.len() {
                return TestResult::discard();
            }

            select(&mut x, k);
            let element = x[k];
            x.sort();
            TestResult::from_bool(element == x[k])
        }
        quickcheck::quickcheck(run as fn(Vec<i32>, usize) -> TestResult)
    }

    #[test]
    fn smoke() {
        for k in 0..4 {
            let mut array = [2, 3, 0, 1];
            select(&mut array, k);
            assert_eq!(array[k], k);
        }
    }
    #[test]
    fn huge() {
        let mut rng = XorShiftRng::new_unseeded();
        for _ in 0..20 {
            let length = rng.gen_range(2_000, 10_000);
            let v: Vec<_> = rng.gen_iter::<i32>().take(length).collect();
            for _ in 0..10 {
                let mut w = v.clone();
                select(&mut w, rng.gen_range(0, length));
            }
        }
    }
}

#[cfg(all(test, feature = "experimental"))]
mod benchs {
    extern crate test;
    use rand::{XorShiftRng, Rng};
    use super::select;

    const N: usize = 2_000;

    #[bench]
    fn huge(b: &mut test::Bencher) {
        let v = XorShiftRng::new_unseeded().gen_iter::<i32>().take(N).collect::<Vec<_>>();
        b.iter(|| {
            let mut w = v.clone();
            select(&mut w, 1000);
        });
    }

    #[bench]
    fn huge_sort(b: &mut test::Bencher) {
        let v = XorShiftRng::new_unseeded().gen_iter::<i32>().take(N).collect::<Vec<_>>();
        b.iter(|| {
            let mut w = v.clone();
            w.sort();
        });
    }
}
