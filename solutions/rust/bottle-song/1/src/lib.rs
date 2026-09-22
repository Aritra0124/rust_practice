pub fn recite(start_bottles: u32, take_down: u32)-> String{
    let string_list = ["No", "One", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight", "Nine", "Ten"];
    let text_one = "{} green bottles hanging on the wall,\n".to_string();
    let mut text_two = "And if one green bottle should accidentally fall,\n".to_string();
    let text_three = "There'll be {} green bottles hanging on the wall.\n\n".to_string();
    let end_bottles = start_bottles - take_down;
    let mut take_down_counter = 0;
    let mut k: String= String::new();
    for i in (end_bottles..=start_bottles).rev() {
        if take_down_counter == take_down{
            break;
        }else{
            take_down_counter += 1;
        }
        if i -1 == 0{
            k.push_str(text_one.replace("bottles", "bottle").replace("{}",string_list[i as usize]).repeat(2).as_mut_str());
            k.push_str(text_two.as_mut_str());
            k.push_str(text_three.replace("{}",&string_list[(i -1) as usize].to_lowercase()).as_mut_str());

            break;
        }else{
            k.push_str(text_one.replace("{}",string_list[i as usize]).repeat(2).as_mut_str());
            k.push_str(text_two.as_mut_str());
            if i - 1 == 1{            k.push_str(text_three.replace("bottles", "bottle").replace("{}",&string_list[(i -1) as usize].to_lowercase()).as_mut_str());
            }else{
                k.push_str(text_three.replace("{}",&string_list[(i -1) as usize].to_lowercase()).as_mut_str());
            }
        }

    }
    k
}