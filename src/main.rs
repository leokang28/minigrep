use std::env;
use std::fs;

struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &Vec<String>) -> Config {
        let query = args[1].clone();
        let filename = args[2].clone();
        Config { query, filename }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    // let (query, filename) = parse_config(&args);
    // let config = Config::new(&args);
    // println!("query: {}", config.query);
    // println!("filename: {}", config.filename);

    // let contents = fs::read_to_string(config.filename).expect("read file failed");

    // println!("text: \n{}", contents);

    let str = String::from("123123");

    pt(&str);
}

fn pt(str: &String) {
    println!("{}", str);
}
