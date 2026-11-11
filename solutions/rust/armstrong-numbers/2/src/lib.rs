pub fn is_armstrong_number(num: u32) -> bool {
    let digits: Vec<u32> = num.to_string().chars().map(|c|c.to_digit(10).unwrap()).collect();
    let num_len = digits.len();
    let mut sum = 0;
    for i in digits{
        sum += i.pow(num_len as u32);
    }
    if sum != num{
        return false;
    }
    true
}
