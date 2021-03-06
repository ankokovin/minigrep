use std::env;
use std::error::Error;
use std::fs;

// TODO(#3): make config an enum for extendability (example - regex)
pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Expected 2 arguments");
        } else if args.len() > 3 {
            eprintln!("Ignored arguments:");
            for idx in 2..args.len() {
                eprint!("{}", args[idx])
            }
            eprintln!();
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        // TODO(#4): allow to pass CASE_INSENSITIVE also from arguments
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }
}

pub fn run<'a>(config: Config) -> Result<(), Box<dyn Error>> {
    // TODO(#5): maybe allow to use regex

    let contents = fs::read_to_string(config.filename)?;
    let result = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in result {
        println!("{}", line);
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            result.push(line);
        }
    }

    result
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();
    let query_lower = query.to_lowercase();
    for line in contents.lines() {
        if line.to_lowercase().contains(&query_lower) {
            result.push(line);
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    // TODO(#6): add unit tests for run
    // TODO(#7): add unit tests for Config::new()
    // TODO(#8): add integration tests
    // TODO(#9): maybe move tests to separate file

    #[test]
    fn one_result() {
        let query = "duct";
        let content = "\
Rust:
safe, fast, productive.
Pick three.";
        assert_eq!(vec!["safe, fast, productive."], search(query, content));
    }

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let content = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape";
        assert_eq!(vec!["safe, fast, productive."], search(query, content));
    }

    #[test]
    fn case_insensitive() {
        let query = "duct";
        let content = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape";
        assert_eq!(
            vec!["safe, fast, productive.", "Duct tape"],
            search_case_insensitive(query, content)
        );
    }
}
