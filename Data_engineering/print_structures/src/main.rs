use clap::Parser;
use print_structures::{
    create_fruit_salad,
    init_languages,
};


#[derive(Parser)]
#[clap(
    version="1.0",
    author="Your Name < your.email@example.com",
    about = "Number of fuirts to include in the salad"
)]

struct Opts {
    #[clap(short, long)]
    number: usize,
}


fn main() {

    // let opts: Opts = Opts::parse();
    // let salad = create_fruit_salad(opts.number);
    // println!("Fruit salad: {:?}", salad);

    // calculate  how long each language has been around
    let languages = init_languages();
    for (language, year) in languages {
        let age = 2025 - year;
        println!("{} : {} ", language, age);
    }
   
}