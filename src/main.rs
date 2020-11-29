use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args);

    println!("query: {}\nfilename: {}", config.query, config.filename);

    let contents = fs::read_to_string(config.filename)
        .expect("a problem reading");
    println!("contents:\n{}", contents);
}

struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Config {
        if args.len() < 3 {
            panic!("execting 2 arguments only found {}", args.len())
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        
        Config { query, filename }
    }
}