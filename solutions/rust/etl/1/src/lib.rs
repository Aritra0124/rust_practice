use std::collections::BTreeMap;

pub fn transform(h: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    let points:Vec<_> = h.keys().cloned().collect();
    let mut new_btree = BTreeMap::new();
    for point in points.iter(){
    for i in h.get(point).unwrap().into_iter().cloned() {
        new_btree.insert(i.to_ascii_lowercase(),point.clone());
    }
    }
    new_btree
}