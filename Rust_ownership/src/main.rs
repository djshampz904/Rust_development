fn main() {
    // comemnt section
    
    let mut s = String::from("Hello");

    s.push_str(", World!");

    println!("{s}");

    moving_variable(5);

    non_moving_variable();

    scope_assignment();

    string_copy();
}

fn moving_variable(num: i32) {
    let x = num;
    let y = x;

    println!("Now we have both value of x: {x} and y: {y}");

}

fn non_moving_variable() {
    let s1 = String::from("hello");
    let s2 = s1;


    println!("But we can access s2: {s2}");

}

fn scope_assignment() {
    let mut s = String::from("Hello");

    s = String::from("Ahoy");

    println!("{s}, world!");

}

fn string_copy() {

    let s1 = String::from("Hello");

    let s2 = s1.clone();

    println!("Now we have both S1: {s1}, S2: {s2}");

}

