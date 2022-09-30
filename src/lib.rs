use std::fs;
use std::error::Error;

pub fn search<'a>(keyword: &str, content: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();

    for line in content.lines() {
        if line.contains(keyword) {
            result.push(line);
        }
    }
    result
}

pub fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(&config.file_path)?;

    for line in search(&config.query_string, &content) {
        println!(">{line}");
    }

    Ok(())
}

#[derive(Debug)]
pub struct Config {
    query_string: String,
    file_path: String
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Need 2 or more arguments");
        }

        let query_string = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config {
            query_string: query_string,
            file_path: file_path
        })
    }
}

