pub fn find(array: &[i32], key: i32) -> Option<usize> {
    if array.is_empty(){
        return None;
    }
    let mut start = 0 as isize;
    let mut end = (array.len() - 1) as isize;
    while start <= end{
        
        let mut mid = start + (end - start) / 2 ;
        if array[mid as usize] == key{
            println!("found it");
            return Some(mid as usize);
        }
        else if array[mid as usize] < key{
            start = mid + 1;
        }else if array[mid as usize] > key{
            end = mid - 1;
        }else{
            return Some(0)
        }
    }
    None
}
