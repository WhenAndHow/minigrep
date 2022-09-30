use std::env;
use std::process::exit;
use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).expect("Error to build config");

    if let Err(e) = minigrep::run(&config) {
        println!("Application error {}", e);
        exit(1);
    }

}

