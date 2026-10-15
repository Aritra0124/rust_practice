use std::collections::HashMap;

pub fn check(candidate: &str) -> bool {
    let mut tracker = HashMap::new();
    let data = candidate.replace("-", "").replace(" ", "").to_lowercase().trim().to_string();
    for i in data.chars() {
        if tracker.contains_key(&i) {
            return false;
        }
        *tracker.entry(i).or_insert(0) += 1;
    }
    println!("{:?}", tracker);
    true
}