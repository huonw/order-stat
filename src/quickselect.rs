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
    let mut a = 1;
    let mut b = x.len() - 1;
    
    'outer: loop {
        loop {
            if a > b { break 'outer; }
            if x[a] >= x[0] { break; }
            a += 1;
        }
        while x[0] < x[b] {
            b -= 1;
        }
        if a >= b { break; }
        
        x.swap(a, b);
        a += 1;
        b -= 1;
    }
    x.swap(0, a - 1);
    a - 1
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

#[cfg(all(test, feature = "unstable"))]
mod benches {
    make_benches!(|m, mut v| super::select(&mut v, m));
}
