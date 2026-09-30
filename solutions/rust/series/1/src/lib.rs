pub fn series(digits: &str, len: usize) -> Vec<String> {
    let mut series = Vec::new();
    let mut start =0;
    let mut end =len;
    while end <= digits.len(){
        series.push(digits.get(start..end).unwrap().to_string());
        start += 1;
        end += 1;
    }
    series
}
