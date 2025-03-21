use std::collections::HashMap;

fn main() {
    let mut new_hash = HashMap::new();

    new_hash.insert(String::from("Hello"), 10);
    new_hash.insert(String::from("Another"), 23);

    println!("{new_hash:?}");

    let tag = String::from("Hello");

    let score = new_hash.get(&tag).copied().unwrap_or(0);

    println!("{score:?}");

    for (key, value) in &new_hash {
        println!("{key}: {value}");
    }


}
