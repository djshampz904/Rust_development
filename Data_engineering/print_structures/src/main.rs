use clap::Parser;
use print_structures::create_fruit_salad;


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

    let opts: Opts = Opts::parse();
    let salad = create_fruit_salad(opts.number);
    println!("Fruit salad: {:?}", salad);

    
   
}