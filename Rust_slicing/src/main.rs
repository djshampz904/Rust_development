fn main() {

    let s1 = String::from("The quick brown fox jumped");

    let frst_wrd = first_word(&s1);

    println!("The first word is: {frst_wrd}");

}

fn first_word(some_string: &str) -> &str {
    let bytes = some_string.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' '{
            return &some_string[0..i];
        }
    }

    &some_string[..]
}   
