use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::HashMap;

pub fn create_fruit_salad(num_fruits: usize) -> Vec<String> {
    let fruits: Vec<String> = vec![
        "apple".to_string(),
        "banana".to_string(),
        "cherry".to_string(),
        "date".to_string(),
        "elderberry".to_string(),
        "fig".to_string(),
        "grape".to_string(),
        "honeydew".to_string(),
        "kiwi".to_string(),
        "lemon".to_string(),
        "mango".to_string(),
        "nectarine".to_string(),
        "orange".to_string(),
        "pear".to_string(),
    ];

    let mut rng = thread_rng();
    let mut fruit_salad = fruits;
    fruit_salad.shuffle(&mut rng);

    fruit_salad.into_iter().take(num_fruits).collect()
}

pub fn init_languages() -> HashMap<String, i32> {
    let mut languages: HashMap<String, i32> = HashMap::new();

    languages.insert("Rust".to_string(), 2010);
    languages.insert("Python".to_string(), 1991);
    languages.insert("Java".to_string(), 1995);
    languages.insert("Go".to_string(), 2009);
    languages.insert("C++".to_string(), 1985);
    languages.insert("C".to_string(), 1972);
    languages.insert("Ruby".to_string(), 1995);
    languages.insert("JavaScript".to_string(), 1995);
    languages.insert("TypeScript".to_string(), 2012);
    languages.insert("Kotlin".to_string(), 2011);
    languages.insert("Swift".to_string(), 2014);
    languages.insert("Scala".to_string(), 2003);
    languages.insert("R".to_string(), 1993);
    languages.insert("Perl".to_string(), 1987);
    languages.insert("PHP".to_string(), 1995);
    languages.insert("Haskell".to_string(), 1990);

    languages

}