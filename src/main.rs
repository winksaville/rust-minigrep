use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = parse_config(&args);

    println!("query: {}\nfilename: {}", config.query, config.filename);

    let contents = fs::read_to_string(config.filename).expect("a problem reading");
    println!("contents:\n{}", contents);
}

struct Config {
    query: String,
    filename: String,
}

fn parse_config(args: &[String]) -> Config {
    let query = args[1].clone();
    let filename = args[2].clone();
    
    Config { query, filename }
}