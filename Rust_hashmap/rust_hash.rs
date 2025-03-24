use std::collections::HashMap;

fn main () {

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Red"), 12);

    println!("Scores: {:?}", scores);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);

    println!("Team: {}, Score: {}", team_name, score);

    
    //iterating over HashMaps
    for (key, value) in &scores {
        println!("Key: {}, Value: {}", key, value);
    }

    let another_record = String::from("Orange");
    let another_score = 45;

    scores.insert(another_record, another_score);

    // println!("{} {}", another_record, another_score);

}
