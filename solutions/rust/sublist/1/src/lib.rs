#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist(first_list: &[i32], second_list: &[i32]) -> Comparison {
    let is_sub = is_sublist(first_list, second_list);
    let is_super = is_sublist(second_list, first_list);

    match (is_sub, is_super) {
        (true, true) => Comparison::Equal,
        (true, false) => Comparison::Sublist,
        (false, true) => Comparison::Superlist,
        (false, false) => Comparison::Unequal,
    }
}

/// Helper function to check if `a` is contained anywhere inside `b`.
fn is_sublist(a: &[i32], b: &[i32]) -> bool {
    if a.is_empty() {
        return true; // An empty list is a sublist of any list
    }
    if a.len() > b.len() {
        return false;
    }

    // Creates sliding windows of size `a.len()` over `b` and checks for equality
    b.windows(a.len()).any(|window| window == a)
}