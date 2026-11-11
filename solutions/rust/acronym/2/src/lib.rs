pub fn camel_case_check(input:&str)->Vec<usize>{
    if input.chars().all(|c| c.is_uppercase()){return vec![0]}
    input.char_indices().filter_map(|(index, c)| if c.is_uppercase(){Some(index)}else { None }).collect()
}
pub fn abbreviate(phrase: &str) -> String {
    let words: Vec<_> = phrase.split([' ', '-']).filter(|s|!s.is_empty()).collect();
    let mut abbreviated = String::new();
    for word in words {
        let pos = camel_case_check(word);
        if !pos.is_empty() {
            for i in pos{
                abbreviated.push(word.to_uppercase().chars().nth(i).unwrap());
            }
        }else if word != "-"{
            abbreviated.push(word.to_uppercase().chars().nth(0).unwrap());
        }
    }
    abbreviated
}