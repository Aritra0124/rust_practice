pub fn egg_count(display_value: u32) -> usize {
    let mut counter = 0;
    let mut val = display_value;
    while val > 0{
        if val % 2 == 1{
            counter += 1;
        }
        val /= 2;
    }
    counter as usize
}
