pub fn square_of_sum(n: u32) -> u32 {
    let mut k = 0;
    for i in 1..=n{
        k += i;
    }
    k.pow(2)
}

pub fn sum_of_squares(n: u32) -> u32 {
    let mut k = 0;
    for i in 1..=n{
        k += i.pow(2);
    }
    k
}

pub fn difference(n: u32) -> u32 {
    square_of_sum(n) - sum_of_squares(n)
}
