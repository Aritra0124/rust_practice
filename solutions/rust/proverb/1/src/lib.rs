pub fn build_proverb(list: &[&str]) -> String {
    let mut full_string = String::new();
    let mut one_line = "For want of a {} the [] was lost.\n".to_string();
    let last_line = "And all for the want of a {}.".to_string();
    if list.len() == 0{
        return full_string;
    }
    else if list.len() > 1{
        for i in 0..list.len()-1{
            full_string.push_str(&one_line.replace("{}", list[i]).replace("[]", list[i+1]));
        }
    }
    full_string.push_str(&last_line.replace("{}", list[0]));
    full_string
}