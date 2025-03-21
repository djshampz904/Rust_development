fn main() {
    let my_vec: Vec<i32> = Vec::new();

    let v = vec![1, 2, 3];

    println!("{v:?}");

    let third: Option<&i32> = v.get(2);

    match third {
        Some(third) => println!("Third element found"),
        None => println!("No third element found"),
    }  

}
