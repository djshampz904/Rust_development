use rand::seq::SliceRandom;
use rand::thread_rng;

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