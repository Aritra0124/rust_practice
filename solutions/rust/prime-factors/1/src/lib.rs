

pub fn factors(n: u64) -> Vec<u64> {
    let mut num = n;
let mut factors = Vec::new();
    while num%2 == 0{
        factors.push(2);
        num /= 2;
    }
let mut factor = 3;
    while factor * factor <= num{
        while num%factor == 0{
            factors.push(factor);
            num /=factor;
        }
        factor += 2;
    }
    if num > 1{
        factors.push(num);
    }
    factors
}
