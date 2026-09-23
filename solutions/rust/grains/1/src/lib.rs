pub fn square(s: u32) -> u64 {
    if s > 64{
        panic!("not possible");
    }
    let mut k: i64 = 2;
    if s == 1{
        return 1;
    }
    k = k.wrapping_pow(s-1);
    k as u64
}

pub fn total() -> u64 {
    let mut k : i64 = 0;
    k = 2_i64.wrapping_pow(64+1)-1;
    k as u64
}
