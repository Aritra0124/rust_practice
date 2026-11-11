pub fn square(s: u32) -> u64 {
    if s > 64{
        panic!("not possible");
    }
    else if s == 1{
        return 1;
    }
    else{
        (2_i64.wrapping_pow(s-1)) as u64
    }
}

pub fn total() -> u64 {
    (2_i64.wrapping_pow(64+1)-1) as u64
}
