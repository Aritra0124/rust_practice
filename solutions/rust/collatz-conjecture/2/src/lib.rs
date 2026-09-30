pub fn odd_check(n: u64) -> bool{
    n.is_multiple_of(2)
}

pub fn collatz(n: u64) -> Option<u64> {
    let mut counter = 0;
    let mut num = n;
    if n == 0{
        return None;
    }else{
    while num != 1{
        if odd_check(num){
            num /= 2
        }else{
            num = (num * 3) + 1;
        }
        counter += 1;
    }}
    Some(counter)
}
