use std::collections::HashMap;

fn main() {
    create_immutable_vector();
    create_mutable_vector();

    let fruit_map = create_fruit_map();

    println!("{:?}", fruit_map);

    if let fruit  = check_fruit("apple") {
        let amount = get_fruit_amount("apple");
        println!("apple found with amount: {}", amount);
    } else {
        println!("apple not found");
    }

    let fruits = vec!["apple", "banana", "apple", "cherry", "banana", "date"];
    let unique_fruits = unique_fruits(fruits);
    println!("unique_fruits: {:?}", unique_fruits);
}

fn create_immutable_vector() {
    let v = vec![1, 3, 5, 7, 9];
    println!("v: {:?}", v);
}

fn create_mutable_vector() {
    let mut y = Vec::new();

    y.push(1);
    y.push(3);
    y.push(5);
    y.push(7);

    println!("y: {:?}", y);
}

fn create_fruit_map() -> HashMap<&'static str, i32> {
    let mut fruit_vector = HashMap::new();

    fruit_vector.insert("apple", 3);
    fruit_vector.insert("banana", 5);
    fruit_vector.insert("cherry", 7);
    fruit_vector.insert("date", 9);

    fruit_vector
}

fn check_fruit(fruit: &str) -> bool {
    let fruit_map = create_fruit_map();

    fruit_map.contains_key(fruit)
}

fn get_fruit_amount(fruit: &str) -> i32 {
    let fruit_map = create_fruit_map();

    match fruit_map.get(fruit) {
        Some(amount) => *amount,
        None => 0,
    }
}

fn unique_fruits(fruits: Vec<&str>) -> Vec<&str> {
    let mut unique_fruits = Vec::new();

    for fruit in fruits {
        if !unique_fruits.contains(&fruit) {
            unique_fruits.push(fruit);
        }
    }
    unique_fruits
}