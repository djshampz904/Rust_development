fn main() {
    let coin = Coin::Penny;
    let my_quarter = Coin::Quarter(UsState::Alabama);

    println!("Coin amount is: {}", coin_type(coin));
    println!("Coin amount is: {}", coin_type(my_quarter));

    let y = plus_one(Some(5));
    let x = plus_one(Some(10));
    let z = plus_one(None);

    println!("y: {y:?}, x: {x:?} z: {z:?}");

    let config_max = Some(3u8);
    if let Some(max) = config_max {
        pritinln!("The maximum is configured to be {max}")

}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    Texas,
    Carlifonia,
}

enum Coin {
    Penny,
    Nickle,
    Dime,
    Quarter(UsState),
}

fn coin_type(coin_type: Coin) -> u8 {
    match coin_type {
        Coin::Penny => 1, 
        Coin::Nickle => 5, 
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("The quarter is from {state:?}");
            25
        }
    }
}


fn plus_one(my_num: Option<i32>) -> Option<i32> {

    match my_num {
        None => None,
        Some(num) => Some(num + 1),
    }
}
