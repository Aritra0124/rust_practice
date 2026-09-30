use std::collections::HashMap;

pub fn diagram_validation(diagram: &str) -> bool {
    if diagram.len() >= 4 { true } else { false }
}
pub fn plants(diagram: &str, student: &str) -> Vec<&'static str> {
    let diagram: String = diagram.chars().filter(|c| !c.is_whitespace()).collect();
    println!("{:?}", diagram);
    let mut plant_names = Vec::new();
    let student_names = vec![
        "Alice", "Bob", "Charlie", "David", "Eve", "Fred", 
        "Ginny", "Harriet", "Ileana", "Joseph","Kincaid", "Larry",
    ];
    let plant_list_map = HashMap::from([
        ('C', "clover"),
        ('G', "grass"),
        ('R', "radishes"),
        ('V', "violets"),
    ]);
    if diagram_validation(&diagram) {
        if diagram.len() > 4 {
            let mid = diagram.len() / 2;
            let (left, right) = diagram.split_at(mid);
            let index = student_names.iter().position(|&c| c == student).unwrap();
            let mut student_plant = left.get(index * 2..(index * 2 + 2)).unwrap().to_string();
            student_plant.push_str(right.get(index * 2..(index * 2 + 2)).unwrap());
            for i in student_plant.chars() {
                if let Some(&plant) = plant_list_map.get(&i) {
                    plant_names.push(plant);
                }
            }
            return plant_names;
        }
        for i in diagram.chars() {
            if let Some(&plant) = plant_list_map.get(&i) {
                plant_names.push(plant);
            }
        }
    }
    plant_names
}