use std::collections::HashMap;
use std::thread::scope;

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    if input.is_empty() || worker_count == 0 {
        return HashMap::new();
    }
    let chunk_size = (input.len() + worker_count - 1) / worker_count;
    let mut output = HashMap::new();
    let lowered_value = input.concat().to_lowercase();

    scope(|scope| {
        let mut temp_data = Vec::new();
        for chunk in lowered_value.as_bytes().chunks(chunk_size) {
            let handler = scope.spawn(|| {
let string_chunks = unsafe { std::str::from_utf8_unchecked(chunk) };
                let mut local_map = HashMap::new();
                for char in string_chunks.chars() {
                    println!("outer char: {}", char);
                    if char.is_alphabetic(){
                        println!("inner char: {}", char);
                        if !local_map.contains_key(&char) {
                            local_map.insert(char, 1);
                        } else {
                            let value = local_map.get(&char).unwrap();
                            local_map.insert(char, *value + 1);
                        }
                    }
                }
                local_map
            });
            temp_data.push(handler);
        }
        for data in temp_data{
            let    handle = data.join().unwrap();
            for (key, count) in handle{
                    *output.entry(key).or_insert(0) += count;
            }
        }
    });
    output
}