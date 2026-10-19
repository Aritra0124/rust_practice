pub fn reply(message: &str) -> &str {
    let message = message.trim();
    let mut is_upper = false;
    let mut has_letter = false;
    let mut only_numeric = false;
    if !message.is_empty(){
        only_numeric = message.chars().all(|c|{
           if c.is_numeric() || c.is_ascii_punctuation() || c.is_whitespace(){
               true
           }else{
               false
           }
        });
        if !only_numeric{
            is_upper = message.chars().all(|c| {
                if !c.is_whitespace() && !c.is_ascii_punctuation() && !c.is_numeric(){
                    c.is_uppercase()
                }else{
                    true
                }
            });
            has_letter = message.chars().all(|c|{
                if c.is_alphabetic() || !c.is_numeric() || c.is_ascii_punctuation() || c.is_whitespace(){
                    true
                }
                else{
                    false
                }
            });
            
        }else{
            only_numeric = false;
        }      
    }else {
        is_upper = false;
    }

    if message.ends_with("?") && has_letter && is_upper && !only_numeric{
        "Calm down, I know what I'm doing!"
    } else if message.ends_with("?"){
        "Sure."
    } else if is_upper{
        "Whoa, chill out!"
    } else if message.is_empty() {
        "Fine. Be that way!"
    } else if !has_letter{
        "Whatever."    
    }else {
        "Whatever."
    }

}
