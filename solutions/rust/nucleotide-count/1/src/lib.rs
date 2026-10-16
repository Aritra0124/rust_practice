use std::collections::HashMap;

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    let list = ['A', 'C', 'G', 'T'];
    if nucleotide.to_string().is_empty() || dna.is_empty(){
       return Ok(0);
    }
    for i in dna.chars(){
        if !list.contains(&i)
        {       return Err(i);
        }
    }
    if !dna.contains(nucleotide) || !list.contains(&nucleotide)
    {
        return Err(nucleotide);
    }
    Ok(dna.matches(nucleotide).count())
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    let mut map = HashMap::new();
    map.insert('A', 0);
    map.insert('C', 0);    
    map.insert('G', 0);    
    map.insert('T', 0);
    let list = ['A', 'C', 'G', 'T'];

    for i in dna.chars(){
        println!("{i}");
        if !list.contains(&i){
            return Err(i);
        }
        *map.entry(i).or_default()=count(i, dna)?;
    }
    Ok(map)
}
