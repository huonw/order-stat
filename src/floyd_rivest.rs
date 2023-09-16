use std::cmp::Ordering::{self, Greater, Less};
use std::{cmp, ptr};

pub fn select<T, F>(array: &mut [T], k: usize, mut f: F)
where
    F: FnMut(&T, &T) -> Ordering,
{
    let r = array.len() - 1;
    select_(array, &mut f, 0, r, k)
}

const A: usize = 600;
const B: f32 = 0.5;

fn select_<T, F>(array: &mut [T], cmp: &mut F, mut left: usize, mut right: usize, k: usize)
where
    F: FnMut(&T, &T) -> Ordering,
{
    while right > left {
        if right - left > A {
            let n = (right - left + 1) as f32;
            let i = (k - left + 1) as f32;
            let z = n.ln();
            let s = B * (z * (2.0 / 3.0)).exp();
            let sn = s / n;
            let sd = B * (z * s * (1.0 - sn)).sqrt() * (i - n * 0.5).signum();

            let isn = i * s / n;
            let inner = k as f32 - isn + sd;
            let new_left = cmp::max(left, inner as usize);
            let new_right = cmp::min(right, (inner + s) as usize);

            select_(array, cmp, new_left, new_right, k)
        }

        let mut i = left + 1;
        let mut j = right - 1;
        array.swap(left, k);
        let t_idx = if cmp(&array[left], &array[right]) != Less {
            array.swap(left, right);
            right
        } else {
            left
        };

        // Need to do this without borrowing (but the assertion above ensures this doesn't alias).
        // This code has been modified throughout to use pointer addition rather than
        // `array.get_unchecked_mut(x)` as the latter causes `t` to alias with the `&mut array`
        // borrow that requires.
        let arr_ptr = array.as_mut_ptr();
        // We can be extra sure that we don't borrow `array` here.
        let t = unsafe { &*arr_ptr.add(t_idx) };
        unsafe {
            while cmp(&*arr_ptr.add(i), t) == Less {
                i += 1
            }
            while cmp(&*arr_ptr.add(j), t) == Greater {
                j -= 1
            }
        }

        if i < j {
            // i < j, and i and j move toward each other, so this
            // assertion ensures that all indexing here is in-bounds.
            assert!(j < array.len());

            // FIXME: this unsafe code *should* be unnecessary: the
            // assertions above mean that LLVM could theoretically
            // optimise out the bounds checks, but it doesn't seem to
            // at the moment (it still does not, 2023-07-29).
            unsafe {
                while i < j {
                    ptr::swap(arr_ptr.add(i), arr_ptr.add(j));
                    i += 1;
                    j -= 1;
                    while cmp(&*arr_ptr.add(i), t) == Less {
                        i += 1
                    }
                    while cmp(&*arr_ptr.add(j), t) == Greater {
                        j -= 1
                    }
                }
            }
        }

        if left == t_idx {
            array.swap(left, j);
        } else {
            j += 1;
            array.swap(right, j);
        }
        if j <= k {
            left = j + 1
        }
        if k <= j {
            right = j.saturating_sub(1);
        }
    }
}

#[cfg(test)]
mod tests {
    use quickcheck::{self, TestResult};
    use rand::{Rng, XorShiftRng};

    use super::select;

    #[test]
    fn qc() {
        fn run(mut x: Vec<i32>, k: usize) -> TestResult {
            if k >= x.len() {
                return TestResult::discard();
            }

            select(&mut x, k, Ord::cmp);
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
            select(&mut array, k, Ord::cmp);
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
                select(&mut w, rng.gen_range(0, length), Ord::cmp);
            }
        }
    }
}

#[cfg(all(test, feature = "unstable"))]
mod benches {
    extern crate test;

    make_benches!(|m, mut v| super::select(&mut v, m, Ord::cmp));
}
