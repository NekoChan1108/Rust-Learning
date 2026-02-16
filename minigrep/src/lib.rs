use std::error::Error;
use std::{env, fs};

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_insensitive: bool,
}

impl Config {
    pub fn new(args: &Vec<String>) -> Result<Self, &str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }
        Ok(Self {
            query: args[1].clone(),
            filename: args[2].clone(),
            case_insensitive: env::var("CASE_INSENSITIVE").is_err(),
        })
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }
    results
}

pub fn run(cfg: Config) -> Result<(), Box<dyn Error>> {
    println!("Searching for {}", cfg.query);
    let contents = fs::read_to_string(cfg.filename)?;
    // println!("With text:\n{}", contents);
    let mut idx = 1;
    let results = if cfg.case_insensitive {
        search(&cfg.query, &contents)
    } else {
        search_case_insensitive(&cfg.query, &contents)
    };
    if results.len() == 0 {
        println!("No results found.");
        // eprintln!("No results found for query: {}", cfg.query);
        return Ok(());
    }
    for line in results {
        println!("{}: {}", idx, line);
        idx += 1;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
Safe, Fast, Productive.
Pick Three.
Duct Tape.
";
        assert_eq!(vec!["Safe, Fast, Productive."], search(query, contents))
    }
    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
Safe, Fast, Productive.
Pick Three.
Trust me.
";
        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        )
    }
}
