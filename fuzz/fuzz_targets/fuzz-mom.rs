#![no_main]
#[macro_use] extern crate libfuzzer_sys;
extern crate order_stat;
extern crate order_stat_fuzz;

use std::cmp;

fuzz_target!(|data: &[u8]| {
    let mut vec = order_stat_fuzz::read_u32s(data);
    if vec.len() == 0 {
        return
    }

    let (idx, &mut median) = order_stat::median_of_medians(&mut vec);
    assert_eq!(median, vec[idx]);

    vec.sort();

    let thirty = (vec.len() * 3 / 10).saturating_sub(1);
    let seventy = cmp::min((vec.len() * 7 + 9) / 10, vec.len() - 1);

    assert!(vec[thirty] <= median);
    assert!(median <= vec[seventy]);
});
