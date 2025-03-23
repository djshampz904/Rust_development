fn main() {
    
    let no_number: Option<Option<()>> = Some(None);
    let some_number = Some(43);

    if let Some(num) = some_number {
        println!("Number found {}", num);
    } else {
            println!("no number found");
        }

    if let Some(other_num) = no_number {
        println!("There is a number {:?}", other_num);
    } else {
        println!("No number");
    }
}
