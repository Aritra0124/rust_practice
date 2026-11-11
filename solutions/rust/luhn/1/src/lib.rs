fn check_chars_in_code(code: &str) -> bool {
    for i in code.chars() {
        if i.is_whitespace(){
            continue;
        }
        else if !i.is_digit(10) {
            return false;
        }else {
            continue;
        }
    }
    true
}

pub fn is_valid(code: &str) -> bool {
    if code.len() == 0 || code == "0" || code == " 0" || !check_chars_in_code(code) {
        return false;
    } else {
        let digits: Vec<i32> = code
            .trim()
            .chars()
            .filter_map(|c| c.to_string().parse().ok())
            .collect();
        let mut counter = 0;
        let mut sum: i32 = 0;
        for i in digits.iter().rev() {
            counter += 1;
            if counter % 2 == 0 {
                if i * 2 > 9 {
                    sum = sum.wrapping_add(i * 2 - 9);
                // println!("counter:{counter} -> value: {}",i*2 - 9);
                } else {
                    sum = sum.wrapping_add(i * 2);
                    // println!("counter:{counter} -> value: {}",i*2);
                }
            } else {
                sum = sum.wrapping_add(*i);
                // println!("counter:{counter} -> value: {}",i);
            }
        }
        if sum % 10 == 0 {
            return true;
        } else {
            return false;
        }
    }
}