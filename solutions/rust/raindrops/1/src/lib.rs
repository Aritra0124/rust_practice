pub fn raindrops(n: u32) -> String {
    let mut song = Vec::new();
    if n% 3 == 0{
        song.push("Pling");
    }
    if n%5 == 0{
        song.push("Plang");
    }
    if n%7 == 0{
        song.push("Plong");
    }
    if !song.is_empty(){
        return song.concat() as String;
    }
    n.to_string()
}
