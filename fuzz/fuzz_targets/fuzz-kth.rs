#![no_main]
#[macro_use] extern crate libfuzzer_sys;
extern crate order_stat;
extern crate order_stat_fuzz;

fuzz_target!(|data: &[u8]| {
    if let Some((mut vec, k)) = order_stat_fuzz::read_u32s_k(data) {
        let computed = *order_stat::kth(&mut vec, k);
        vec.sort();
        let actual = vec[k];
        assert_eq!(computed, actual);
    }
});
