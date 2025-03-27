use rand::seq::SliceRandom;
use rand::thread_rng;
use std::cmp::Ord;
use std::collections::BinaryHeap;


#[derive(Debug, Eq, PartialEq)]
enum Fruit {
    Fig,
    Other(String),
}

impl Ord for Fruit {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Fruit::Fig, Fruit::Fig) => std::cmp::Ordering::Equal,
            (Fruit::Fig, Fruit::Other(_)) => std::cmp::Ordering::Greater,
            (Fruit::Other(_), Fruit::Fig) => std::cmp::Ordering::Less,
            (Fruit::Other(_), Fruit::Other(_)) => std::cmp::Ordering::Equal,
        }
    }
}

impl PartialOrd for Fruit {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn generate_fruit_salad() -> BinaryHeap<Fruit> {
    let mut rng = thread_rng();

    let fruits = vec![
        "Apple",
        "Banana",
        "Orange",
        "Fig",
        "Fig",
        "Pineapple",
        "Strawberry",
        "Kiwi",
        "Mango",
        "Peach",
        "Plum",
        "Grape",
        "Fig",
        "Fig",
        "Cherry",
        "Blueberry",
        "Raspberry",
        "Blackberry",
        "Cranberry",
        "Gooseberry",
        "Currant",
        "Fig",
        "Elderberry",
    ];

    let mut fruit_salad: BinaryHeap<Fruit> = BinaryHeap::new();

    let mut fig_count = 0;

    while fig_count < 2 {
        let fruit = fruits.choose(&mut rng).unwrap();
        
        if *fruit == "Fig" {
            fig_count += 1;

            fruit_salad.push(Fruit::Fig);
        } else {
            fruit_salad.push(Fruit::Other(fruit.to_string()));
        }
    }

    fruit_salad

}

fn main () {
    let fruitsalad = generate_fruit_salad();

   for item in fruitsalad {
        match item {
            Fruit::Fig => println!("Fig"),
            Fruit::Other(fruit) => println!("{}", fruit),
        }
   }
}

