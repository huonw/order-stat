extern crate byteorder;

use byteorder::{ByteOrder, LittleEndian};

pub fn read_u32s(data: &[u8]) -> Vec<u32> {
    let u32s = data.len() / 4;
    let mut vec = vec![0u32; u32s];
    LittleEndian::read_u32_into(&data[..u32s * 4], &mut vec);

    vec
}

pub fn read_u32s_k(data: &[u8]) -> Option<(Vec<u32>, usize)> {
    if data.len() < 2 {
        return None
    }

    let (first, rest) = data.split_at(2);
    let k = LittleEndian::read_u16(first) as usize;
    let vec = read_u32s(rest);
    if k >= vec.len() {
        return None
    }

    Some((vec, k))
}
