use colored::*;
use std::error::Error;
use std::fs;
pub struct Config {
    pattern: String,
    input: String,
    sensitive: bool,
}

impl Config {
    pub fn new(matches: clap::ArgMatches) -> Config {
        Config {
            pattern: matches.value_of("pattern").unwrap().to_string(),
            input: matches.value_of("input").unwrap().to_string(),
            sensitive: matches.is_present("sensitive"),
        }
    }

    pub fn run(&self) -> Result<(), Box<dyn Error>> {
        let contents = fs::read_to_string(&self.input)?;

        let search_fn = match self.sensitive {
            true => search,
            false => search_case_insensitive,
        };

        for line in search_fn(&self.pattern, &contents) {
            println!(
                "{}",
                line.replace(&self.pattern, &self.pattern.on_yellow().black().to_string())
            );
        }

        Ok(())
    }
}

fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let pattern = "duct";
        let input = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(pattern, input));
    }

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
