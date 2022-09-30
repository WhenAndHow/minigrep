use std::env;
use std::fs;

fn main() {
    let args = env::args().collect();

    let config = Config::new(args);

    dbg!(config);

}

fn run(config: Config) -> Result<()> {
    fs::
}

#[derive(Debug)]
struct Config {
    query_string: String,
    file_path: String
}

impl Config {
    fn new(args: Vec<String>) -> Config {
        if args.len() < 3 {
            panic!("Need 2 or more arguments");
        }

        let query_string = args[1].clone();
        let file_path = args[2].clone();

        Config {
            query_string: query_string,
            file_path: file_path
        }
    }
}

