use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert("A", 30);
    map.insert("B", 50);

    match map.get("D") {
        Some(v) => println!("D={}", v),
        None => println!("There're no D"),
    }
}
