/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    if isbn.len() < 10{
        return false;
    }
    let list = isbn.replace("-", "");
    let len = list.len();
    let end = list.chars().last().unwrap();
    if len != 10 || (!end.is_ascii_digit() && end != 'X') {
        return false
    }
    let mut summation = 0;
        for (i, c) in list.chars().enumerate() {
        let val = if c == 'X' && i == len-1{
            10
        } else if let Some(digit) = c.to_digit(10) {
            digit as i32
        } else {
            return false;
        };
        println!("{:?}: {:?} => {:?}", i, c, val);
        summation += (10 - i as i32) * val;
    }
    if summation % 11 == 0{
        true
    }
    else {
        false
    }
}
