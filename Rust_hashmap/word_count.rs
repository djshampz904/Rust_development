use std::collections::HashMap;

fn main () {


    let mut hash_map = HashMap::new();


    let my_string = "Hello Hello world there there there are you you will";

    for i in my_string.split_whitespace() {
        let count = hash_map.entry(i).or_insert(0);

        *count += 1
    }
    
    println!("{:?}", hash_map);
}

