pub fn translate(rna: &str) -> Option<Vec<&str>> {
    let mut output:Vec<&str> = Vec::new();
    if !rna.is_empty() && rna.len() < 3{
        return None;
    }
    if !rna.len().is_multiple_of(3) && (!rna.contains("UAA") && !rna.contains("UAG") && !rna.contains("UGA")){
        return None;
    }
    if rna.len() == 3 && (!rna.contains("U") && !rna.contains("A") && !rna.contains("C") && !rna.contains("G")) {
        return None;
    }
    let list: Vec<String> = rna.chars().collect::<Vec<char>>().chunks(3).map(|chunk| chunk.iter().collect()).collect();
    println!("{:?}", list);
    for i in list{
        match i.as_str(){
            "AUG" => output.push("Methionine"),
            "UUU" | "UUC" => output.push("Phenylalanine"),
            "UUA" | "UUG" => output.push("Leucine"),
            "UCU" | "UCC" | "UCA" | "UCG" => output.push("Serine"),
            "UAU" | "UAC" => output.push("Tyrosine"),
            "UGU" | "UGC" => output.push("Cysteine"),
            "UGG" => output.push("Tryptophan"),
            "UAA" | "UAG" | "UGA" => break,
            _ => break,
        }
    }
    Some(output)
}
