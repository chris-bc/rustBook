use std::fs;
use std::error::Error;
use std::env;

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(
        mut args: impl Iterator<Item = String>,
    ) -> Result<Config, &'static str> {
        let mut ignore_case = false;
        args.next();

        let query = match args.next() {
            Some(arg) => {
                if arg == "-i".to_string() {
                    ignore_case = true;
                    match args.next() {
                        Some(arg) => arg,
                        None => return Err("Ignoring case but didn't get a query string\nUsage: minigrep [-i] <pattern> <filename>"),
                    }
                } else {
                    ignore_case = env::var("IGNORE_CASE").is_ok();
                    arg
                }
            },
            None => return Err("Didn't get a query string\nUsage: minigrep [-i] <pattern> <filename>"),
        };
        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path\nUsage: minigrep [-i] <pattern> <filename>"),
        };

        Ok(Config { query, file_path, ignore_case })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
        println!("{line}");
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(
    query: &str,
    contents: &'a str,
) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.to_string()
                                    .to_lowercase()
                                    .contains(query.to_string()
                                                    .to_lowercase()
                                                    .as_str()))
        .collect()
//     let query = query.to_lowercase();
//     let mut results = Vec::new();

//     for line in contents.lines() {
//         if line.to_lowercase().contains(&query) {
//             results.push(line);
//         }
//     } 
//     results
}

#[cfg(test)]
mod tests {
    use super::*;

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