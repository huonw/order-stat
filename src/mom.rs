use std::{cmp, mem};
use std::cmp::Ordering;

/// Calculate an approximate median of `array`.
///
/// The return value is the index/reference to some value of `array`
/// that is guaranteed to lie between the 30th and 70th percentiles of
/// the values in `array`. That is, it both is not smaller and not
/// larger than than at least 30% of the elements of `array`.
///
/// This is equivalent to `median_of_medians_by(array, Ord::cmp)`.
///
/// # Panics
///
/// This panics if `array` is empty.
///
/// # Examples
///
/// ```rust
/// // the numbers 0, 1, ..., 100.
/// let mut v = (0..101).rev().collect::<Vec<_>>();
/// let (_, &mut median) = order_stat::median_of_medians(&mut v);
/// assert!(30 <= median);
/// assert!(median <= 70);
/// ```
pub fn median_of_medians<T: Ord>(array: &mut [T]) -> (usize, &mut T) {
    median_of_medians_by(array, Ord::cmp)
}

/// Calculate an approximate median of `array`, using the ordering
/// defined by `cmp`.
///
/// The return value is the index/reference to some value of `array`
/// that is guaranteed to lie between the 30th and 70th percentiles of
/// the values in `array`. That is, the return value is such that
/// `cmp` will return `Greater` for at most 70% of the elements and
/// similarly will return `Less` for at most 70%.
///
/// # Panics
///
/// This panics if `array` is empty.
///
/// # Examples
///
/// ```rust
/// // the numbers 0.0, 1.0, ..., 100.0.
/// let mut v = (0..101).map(|x| x as f64).rev().collect::<Vec<_>>();
///
/// let (_, &mut median) = order_stat::median_of_medians_by(&mut v, |x, y| x.partial_cmp(y).unwrap());
/// assert!(30.0 <= median);
/// assert!(median <= 70.0);
/// ```
pub fn median_of_medians_by<T, F>(array: &mut [T], mut cmp: F) -> (usize, &mut T)
    where F: FnMut(&T, &T) -> Ordering
{
    if array.len() < 5 {
        let median = array.len() / 2;
        return (median, super::kth_by(array, median, cmp))
    }
    let num_medians = (array.len() + 4) / 5;
    for i in 0..num_medians {
        let start = 5 * i;
        let trailing = array.len() - start;
        let idx = if trailing < 5 {
            let elem = super::kth_by(&mut array[start..], trailing / 2, &mut cmp) as *mut _ as usize;

            // compute the index of that element (zero sized types
            // don't matter what index they end up, they're all at the
            // same location and hence indistinguishable).
            let start = array.as_ptr() as usize;
            (elem - start) / cmp::max(1, mem::size_of::<T>())
        } else {
            start + median5(&array[start..start+5], &mut cmp)
        };
        array.swap(i, idx);
    }
    let idx = num_medians / 2;
    (idx, super::kth_by(&mut array[..num_medians], idx, cmp))
}

fn median5<T, F>(array: &[T], cmp: &mut F) -> usize
    where F: FnMut(&T, &T) -> Ordering
{
    use std::mem;

    let array = array;
    debug_assert!(array.len() == 5);

    let mut a4 = &array[4];
    let mut a3 = &array[3];
    let mut a2 = &array[2];
    let mut a1 = &array[1];
    let mut a0 = &array[0];

    macro_rules! cmp {
        ($($a: ident, $b: ident;)*) => {
            $(
                if cmp($a, $b) == Ordering::Less {
                    mem::swap(&mut $a, &mut $b)
                }
                )*
        }
    }

    cmp! {
        a1, a0;
        a2, a0;
        a3, a0;
        a4, a0;
        a2, a1;
        a3, a1;
        a4, a1;
        a3, a2;
        a4, a2;
    }

    if mem::size_of::<T>() == 0 {
        0
    } else {
        (a2 as *const _ as usize - array.as_ptr() as usize) / mem::size_of::<T>()
    }
}

#[cfg(test)]
mod tests {
    use std::cmp;
    use super::median_of_medians;
    use quickcheck::{self, TestResult};

    #[test]
    fn qc() {
        fn run(mut x: Vec<i32>) -> TestResult {
            if x.is_empty() { return TestResult::discard() }

            let (_, &mut median) = median_of_medians(&mut x);
            x.sort();

            let thirty = x.len() * 3 / 10;
            let seventy = cmp::min((x.len() * 7 + 9) / 10, x.len() - 1);
            TestResult::from_bool(x[thirty] <= median && median <= x[seventy])
        }
        quickcheck::quickcheck(run as fn(Vec<i32>) -> TestResult)
    }

    #[test]
    fn smoke() {
        let mut x = (0..101).rev().collect::<Vec<_>>();
        let (_, &mut median) = median_of_medians(&mut x);
        assert!(30 <= median);
        assert!(median <= 70);
    }

    #[test]
    fn include_trailing() {
        // there was a bug with this vector of length 14 (!== 0 (mod
        // 5)).
        let mut v = [0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0, 0];
        assert_eq!(*median_of_medians(&mut v).1, 0)
    }
}

#[cfg(all(test, feature = "unstable"))]
mod benches {
    extern crate test;
    use rand::{XorShiftRng, Rng};
    use super::median_of_medians;

    const N: usize = 20_000;

    #[bench]
    fn huge(b: &mut test::Bencher) {
        let v = XorShiftRng::new_unseeded().gen_iter::<i32>().take(N).collect::<Vec<_>>();
        b.iter(|| {
            let mut w = v.clone();
            median_of_medians(&mut w).0
        });
    }

    #[bench]
    fn huge_exact(b: &mut test::Bencher) {
        let v = XorShiftRng::new_unseeded().gen_iter::<i32>().take(N).collect::<Vec<_>>();
        b.iter(|| {
            let mut w = v.clone();
            ::kth(&mut w, N / 2) as *mut _
        });
    }

}
