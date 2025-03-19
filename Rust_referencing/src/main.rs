fn main() {
    let my_string = String::from("Hello World");

    let str_len = calculate_len(&my_string);

    println!("String: {my_string}, length: {str_len}");

    let mut another_string = String::from("Some context");

    change_ref(&mut another_string);

    println!("{another_string}");

}


fn calculate_len(some_string: &String) -> usize {

    let string_len = some_string.len();

    string_len 
}

fn change_ref(some_string: &mut String) {

    some_string.push_str(", More context");

}


