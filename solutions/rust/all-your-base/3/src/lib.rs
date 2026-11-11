#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    InvalidInputBase,
    InvalidOutputBase,
    InvalidDigit(u32),
}

///
/// Convert a number between two bases.
///
/// A number is any slice of digits.
/// A digit is any unsigned integer (e.g. u8, u16, u32, u64, or usize).
/// Bases are specified as unsigned integers.
///
/// Return the corresponding Error enum if the conversion is impossible.
///
///
/// You are allowed to change the function signature as long as all test still pass.
///
///
/// Example:
/// Input
///   number: &[4, 2]
///   from_base: 10
///   to_base: 2
/// Result
///   Ok(vec![1, 0, 1, 0, 1, 0])
///
/// The example corresponds to converting the number 42 from decimal
/// which is equivalent to 101010 in binary.
///
///
/// Notes:
///  * The empty slice ( "[]" ) is equal to the number 0.
///  * Never output leading 0 digits, unless the input number is 0, in which the output must be `[0]`.
///    However, your function must be able to process input with leading 0 digits.
///
pub fn to_base_ten(number: &[u32], to_base: u32)-> Vec<u32>{
    let mut sum = 0;
    for (index,i) in number.iter().enumerate() {
        sum += (*i as i32) * (to_base as i32).pow((number.len() as u32 -1) - (index as u32));
    }
    sum.to_string().chars().map(|c| c.to_digit(10).unwrap()).collect::<Vec<u32>>()
}
pub fn from_base_ten(number: &[u32], to_base: u32) -> Vec<u32> {
    let mut sum = 0;
    let mut result = Vec::new();
    for (index,i) in number.iter().enumerate() {
        sum += (*i as i32) * 10_i32.pow((number.len() as u32 -1) - (index as u32))
    }
    let mut rem = sum as u32;
    while rem >= 1{
        if rem%to_base == 0{
            result.push(0);
        }
        if rem%to_base > 0 && rem%to_base < to_base {
            result.push(rem%to_base);
        }
        rem /= to_base;
    }
    result.reverse();
    result
}

pub fn convert(number: &[u32], from_base: u32, to_base: u32) -> Result<Vec<u32>, Error> {
    let mut result = Vec::new();
    let mut temp_result = Vec::new();
    let base = 10;
    if from_base == 0 || from_base == 1{
        return Err(Error::InvalidInputBase)
    }
    if to_base == 0 || to_base == 1{
        return Err(Error::InvalidOutputBase)
    }
    if number.contains(&from_base){
        return Err(Error::InvalidDigit(from_base))
    }
    if number.iter().all(|n| *n == 0) || number.is_empty(){
        return Ok(vec![0]);
    }
    if from_base != base || to_base != base {
        temp_result.extend(to_base_ten(number, from_base));
    }
    if to_base != base {
        result.extend(from_base_ten(&temp_result, to_base));
    }else{
        result = temp_result;
    }
    println!("{:?}", result);
    Ok(result)
}