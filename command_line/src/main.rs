use std::env;
use std::fs;
use std::process;
use std::error::Error;
use command_line::Config;


fn main() {
    
    let my_args: Vec<String> = env::args().collect();

    let config = Config::new(&my_args).unwrap_or_else(|err| {
        println!("Problem parsing arguments {err}");
        process::exit(1);
    });

    if let Err(e) = command_line::run(config) {
        println!("Application error {e}");
        process::exit(1)
    }
}
