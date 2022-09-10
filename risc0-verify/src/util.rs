pub fn to_po2(x: usize) -> usize {
    (31 - (x as u32).leading_zeros()) as usize
}
