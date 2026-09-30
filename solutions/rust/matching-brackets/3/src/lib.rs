pub fn brackets_are_balanced(string: &str) -> bool {
    let mut stack = Vec::new();
    for i in string.chars(){
        match i {
            '(' | '{' | '[' => stack.push(i),
            ')' => if stack.pop() != Some('(') { return false},
            '}' => if stack.pop() != Some('{') { return false},
            ']' => if stack.pop() != Some('[') { return false},
            _ => (),            
        }
    }
    stack.is_empty()
}
