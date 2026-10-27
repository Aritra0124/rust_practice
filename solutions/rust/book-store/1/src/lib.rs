use std::collections::{HashMap, BTreeMap};
pub fn group_creation(books: &[u32]) -> Vec<Vec<u32>>{
    let mut freq = BTreeMap::new();
    for i in books{
        *freq.entry(i).or_insert(0) += 1;
    }
    let mut result = Vec::new();
    while !freq.is_empty(){
        let mut temp_vec = Vec::new();
        let mut temp_remove_vec = Vec::new();
        for (&key, count) in freq.iter_mut(){
            if *count > 0{
                temp_vec.push(*key);
                *count -= 1;
            }
            if *count == 0{
                temp_remove_vec.push(*key);
            }
        }
        for i in temp_remove_vec{
            freq.remove(&i);
        }
        result.push(temp_vec);
    }
    return result;
}

pub fn lowest_price(books: &[u32]) -> u32 {
    let mut discount_map = HashMap::new();
    discount_map.insert(1, 0);
    discount_map.insert(2, 5);
    discount_map.insert(3, 10);
    discount_map.insert(4, 20);
    discount_map.insert(5, 25);
    
    let mut basket = group_creation(&books);

    loop {
        let mut five_idx = None;
        let mut three_idx = None;

        for (idx, group) in basket.iter().enumerate() {
            if group.len() == 5 && five_idx.is_none() {
                five_idx = Some(idx);
            } else if group.len() == 3 && three_idx.is_none() {
                three_idx = Some(idx);
            }
        }

        if let (Some(f), Some(t)) = (five_idx, three_idx) {
            basket.remove(std::cmp::max(f, t));
            basket.remove(std::cmp::min(f, t));
            basket.push(vec![0; 4]);
            basket.push(vec![0; 4]);
        } else {
            break;
        }
    }
    
    let mut sum = 0;
    for i in basket{
        let book_count = i.len() as u32;
        let discount = discount_map.get(&book_count).unwrap();
        sum += book_count * (100 - discount) * 8;
    }
    return sum as u32;
}
