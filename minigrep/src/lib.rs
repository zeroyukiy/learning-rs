use std::error::Error;
use std::fs;
use std::env;

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();
        let optional_param = if args.len() == 4 {true} else                 
        
        
        
        { false};
        let case_sensitive = if optional_param && args[3] == "--insensitive" {
            false
        } else { env::var("CASE_INSENSITIVE").is_err() };
        println!("{}", case_sensitive);
        Ok(Config { query, filename, case_sensitive })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insentivie(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }

    // println!("With text:\n{}", contents);

    Ok(())
}

#[allow(unused_variables)]
#[allow(dead_code)]
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = vec![];
    // Searching Each Line for the Query
    // Iterate through each line of the contents.
    for line in contents.lines() {
        if line.contains(query) {
            // Storing Matching Lines
            results.push(line)
        }
    }

    results
}

#[allow(unused_variables)]
pub fn search_case_insentivie<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = vec![];

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line)
        }
    }

    results
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

        assert_eq!(vec!["Rust:", "Trust me."], search_case_insentivie(query, contents))
    }

}
