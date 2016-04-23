use test::Bencher;
use rand::{XorShiftRng, Rng};

pub const N_HUGE: usize = 100_000;
pub const N_LARGE: usize = 2_000;
pub const N_SMALL: usize = 20;

pub fn run<T, F: FnMut(usize, Vec<i32>) -> T>(b: &mut Bencher, n: usize, mut f: F) {
    let v = XorShiftRng::new_unseeded().gen_iter::<i32>().take(n).collect::<Vec<_>>();
    let m = n / 5;
    b.iter(|| f(m, v.clone()));
}
#[macro_export]
macro_rules! make_benches {
    ($e: expr) => {
        #[bench]
        fn small(b: &mut ::test::Bencher) {
            ::benches::run(b, ::benches::N_SMALL, $e)
        }
        #[bench]
        fn large(b: &mut ::test::Bencher) {
            ::benches::run(b, ::benches::N_LARGE, $e)
        }
        #[bench]
        fn huge(b: &mut ::test::Bencher) {
            ::benches::run(b, ::benches::N_HUGE, $e)
        }
    }
}

mod sort {
    make_benches!(|_, mut v| v.sort());
}
