pub fn prime_check(k: u32) -> bool{
    if k <2{
        return false;
    }
    for i in 2..k{
        if k%i == 0{
            return false;
        }
    }
    true
}
pub fn nth(n: u32)-> u32 {
    let mut counter = 0;
    let mut pos = 1;
    while counter <= n{
        if prime_check(pos) {
            counter += 1;
            if counter > n{
                return pos;
            }
        }
        pos +=1;
    }
    pos
}