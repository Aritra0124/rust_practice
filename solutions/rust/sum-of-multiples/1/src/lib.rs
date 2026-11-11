pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut points = Vec::new();
    for i in factors.iter(){
        let mut k = 1;
        while i*k < limit{
            points.push(i*k);
            k += 1;
        }
        points.sort_unstable();
        points.dedup();
    }
    points.iter().sum()
}
