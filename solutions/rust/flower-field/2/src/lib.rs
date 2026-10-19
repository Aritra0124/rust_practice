pub fn annotate(garden: &[&str]) -> Vec<String> {
    let matrix: Vec<Vec<char>> = garden
        .iter()
        .map(|row| row.chars().collect())
        .collect();
    let pos:[[i8;2];9] =[[-1, -1], [-1, 0], [-1, 1],
                        [0, -1], [0, 0], [0, 1],
                        [1, -1], [1, 0], [1, 1]];
    let mut output:Vec<String> = Vec::new();
    for i in 0..matrix.len() {
        let mut d = "".to_string();
        for j in 0..matrix[i].len() {
            let mut count = 0;
            let mut flag = false;
            for k in pos {
                if matrix[i][j] == '*' {
                    flag = true;
                }
                else if ((i as i8 + k[0]) > -1) && ((i as i8 + k[0]) < matrix.len() as i8) && ((j as i8 + k[1]) > -1) && ((j as i8 + k[1]) < matrix[i].len() as i8) {
                    if matrix[(i as i8 + k[0]) as usize][(j as i8 + k[1]) as usize] == '*' {
                        count += 1;
                    }
                }
                else{
                    continue;
                }
            }
            if flag{
                d += "*";
            }else if count == 0 {
                d += " ";
            }else {
                d += count.to_string().as_str();
            }
        }
        output.push(d);
    }
    // println!("{:?}", output);
    output
}
