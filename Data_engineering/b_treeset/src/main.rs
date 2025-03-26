use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::BTreeSet;

fn main() {
    let fruits = vec![
        "apple", "banana", "cherry", "date", "elderberry", "fig", "grape", "honeydew", "kiwi",
        "lemon", "mango", "nectarine", "orange", "pear", "quince", "raspberry", "strawberry",
        "tangerine", "ugli", "watermelon",
    ];

    let mut my_fruit_basket = BTreeSet::new();

    let mut rng = thread_rng();
    for _ in 0..10 {
        let fruit = fruits.choose(&mut rng).unwrap();
        my_fruit_basket.insert(fruit);
    };

    println!("My fruit basket contains:");
    for fruit in my_fruit_basket.iter() {
        println!("{}", fruit);
    }

    // select letters between j and q
    let j = my_fruit_basket.range("j".."q");
    println!("Fruits between j and q:");
    for fruit in j {
        println!("{}", fruit);
    }
}