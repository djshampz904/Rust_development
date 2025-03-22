fn main() {

    let mut height = 190;
    height = height - 20;


    let results = if height > 180 {
        "tall"
    } else if height > 170 {
        "Average"
    } else {
        "short"
    };

    let health = if height < 180 {"good"} else {"unknown"};

    println!("Health: {health}");

    println!("Results: {results}");

    let health = if height < 180 {true} else {false};

    println!("health {}", health);


}


