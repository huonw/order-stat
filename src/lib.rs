//! Calculate order statistics.
//!
//! This crates allows one to compute the `k`th smallest element in
//! (expected) linear time, and estimate a median element via the
//! median-of-medians algorithm.
//!
//! [Source](https://github.com/huonw/order-stat)
//!
//! # Installation
//!
//! Ensure your `Cargo.toml` contains:
//!
//! ```toml
//! [dependencies]
//! order-stat = "0.1"
//! ```
//!
//! # Examples
//!
//! ```rust
//! let mut v = [4, 1, 3, 2, 0];
//!
//! println!("the 2nd smallest element is {}", order_stat::kth(&mut v, 1));
//! ```
//!
//! ```rust
//! let mut v = [4, 1, 3, 2, 0];
//!
//! println!("{} is close to the median", order_stat::median_of_medians(&mut v).1);
//! ```

#![cfg_attr(all(test, feature = "unstable"), feature(test))]

#[cfg(test)] extern crate rand;
#[cfg(test)] extern crate quickcheck;
#[cfg(all(test, feature = "unstable"))] extern crate test;

#[cfg(all(test, feature = "unstable"))]
#[macro_use]
mod benches;

mod floyd_rivest;
mod quickselect;
mod mom;

/// Compute the `k`th order statistic (`k`th smallest element) of
/// `array` via the Floyd-Rivest Algorithm[1].
///
/// The return value is the same as that returned by the following
/// function (although the final order of `array` may differ):
///
/// ```rust
/// fn kth_sort<T: Ord>(array: &mut [T], k: usize) -> &mut T {
///     array.sort();
///     &mut array[k]
/// }
/// ```
///
/// That is, `k` is zero-indexed, so the minimum corresponds to `k =
/// 0` and the maximum `k = array.len() - 1`. Furthermore, `array` is
/// mutated, placing the `k`th order statistic into `array[k]` and
/// partitioning the remaining values so that smaller elements lie
/// before and larger after.
///
/// If *n* is the length of `array`, `kth` operates with (expected)
/// running time of *O(n)*, and a single query is usually much faster
/// than sorting `array` (per `kth_sort`). However, if many order
/// statistic queries need to be performed, it may be more efficient
/// to sort and index directly.
///
/// For convenience, a reference to the requested order statistic,
/// `array[k]`, is returned directly. It is also accessibly via
/// `array` itself.
///
/// [1]: Robert W. Floyd and Ronald L. Rivest (1975). Algorithm 489:
/// the algorithm SELECT—for finding the *i*th smallest of *n* elements
/// [M1]. *Commun. ACM* **18**, 3,
/// 173. doi:[10.1145/360680.360694](http://doi.acm.org/10.1145/360680.360694).
///
/// # Panics
///
/// If `k >= array.len()`, `kth` panics.
///
/// # Examples
///
/// ```rust
/// let mut v = [10, 0, -10, 20];
/// let kth = order_stat::kth(&mut v, 2);
///
/// assert_eq!(*kth, 10);
/// ```
///
/// If the order of the original array, or position of the element is
/// important, one can collect references to a temporary before querying.
///
/// ```rust
/// use std::mem;
///
/// let mut v = [10, 0, -10, 20];
///
/// // compute the order statistic of an array of references (the Ord
/// // impl defers to the internals, so this is correct)
/// let kth = *order_stat::kth(&mut v.iter().collect::<Vec<&i32>>(), 2);
///
/// // the position is the difference between the start of the array
/// // and the order statistic's location.
/// let index = (kth as *const _ as usize - &v[0] as *const _ as usize) / mem::size_of_val(&v[0]);
///
/// assert_eq!(*kth, 10);
/// assert_eq!(index, 0);
/// ```
pub fn kth<T: Ord>(array: &mut [T], k: usize) -> &mut T {
    assert!(k < array.len(),
            "order_stat::kth called with k = {} >= len = {}", k, array.len());
    floyd_rivest::select(array, k);
    &mut array[k]
}

pub use mom::median_of_medians;
