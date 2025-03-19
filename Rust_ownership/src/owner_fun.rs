fn main() {
    let s = String::from("hello");

    takes_ownership(s);

    let x = 5;

    makes_copy(x);

    println!("{}", x);

    let s1 = give_ownership();

    let s2 = String::from("New String");

    let s3 = return_ownership(s2);

    println!("The string here is {s3}");

    let s4 = String::from("Hello World");

    let (s4, str_len) = return_multiple_values(s4);

    println!("String: {s4}, \n String length: {str_len}");

    

}

fn takes_ownership(some_string: String) {
    println!("{some_string}");
}

fn makes_copy(some_number: i32) {
    println!("{some_number}");
}

fn give_ownership() -> String {
    
    let my_string = String::from("New string");

    my_string
}

fn return_ownership(some_string: String) -> String {

    some_string
}

fn return_multiple_values(some_string: String) -> (String, usize) {
    
    let string_length = some_string.len();

    (some_string, string_length)

}




