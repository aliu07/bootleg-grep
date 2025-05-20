use std::error::Error;
use std::{env, fs};

pub struct Config {
    query: String,
    file_path: String,
    ignore_case: bool,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        // Skip executable path
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("did not get a query string"),
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("did not get a file path"),
        };

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }

    pub fn get_query(&self) -> &str {
        &self.query
    }

    pub fn get_file_path(&self) -> &str {
        &self.file_path
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(&config.file_path)?;

    let result = if config.ignore_case {
        search_case_insensitive_iter(&config.get_query(), &contents)
    } else {
        search_iter(&config.get_query(), &contents)
    };

    for line in result {
        println!("{line}");
    }

    Ok(())
}

pub fn search_loop<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            result.push(line);
        }
    }

    return result;
}

pub fn search_iter<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive_loop<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut result = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            result.push(line);
        }
    }

    result
}

pub fn search_case_insensitive_iter<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();

    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive_loop() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search_loop(query, contents)
        );
    }

    #[test]
    fn case_sensitive_iter() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search_iter(query, contents)
        );
    }

    #[test]
    fn case_insensitive_loop() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive_loop(query, contents)
        );
    }

    #[test]
    fn case_insensitive_iter() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive_iter(query, contents)
        );
    }
}
