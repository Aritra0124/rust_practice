pub fn raindrops(n: u32) -> String {
    let mut song = Vec::new();
    if n.is_multiple_of(3){
        song.push("Pling");
    }
    if n.is_multiple_of(5){
        song.push("Plang");
    }
    if n.is_multiple_of(7){
        song.push("Plong");
    }
    if !song.is_empty(){
        return song.concat() as String;
    }
    n.to_string()
}
