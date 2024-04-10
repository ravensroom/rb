pub mod minigrep {
    use std::env;
    use std::fs;
    use std::process;

    pub fn run() {
        let args: Vec<String> = env::args().collect();
        let config = Config::build(&args).unwrap_or_else(|err| {
            eprintln!("Problem parsing arguments: {err}");
            process::exit(1);
        });

        let contents = fs::read_to_string(config.file_path).unwrap_or_else(|err| {
            eprintln!("Problem reading the file: {err}");
            process::exit(1);
        });

        let results = if config.ignore_case {
            search_case_insensitive(&config.query, &contents)
        } else {
            search(&config.query, &contents)
        };

        for line in results {
            println!("{line}");
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
        fn test_search() {
            let query = "duct";
            let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

            assert_eq!(vec!["safe, fast, productive."], search(query, contents));
        }

        #[test]
        fn test_search_case_insensitive() {
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

    struct Config {
        query: String,
        file_path: String,
        ignore_case: bool,
    }

    impl Config {
        fn build(args: &[String]) -> Result<Config, &'static str> {
            if args.len() < 3 {
                return Err("Bad arguments! Please provide a pattern and a file path");
            }
            Ok(Config {
                query: args[1].clone(),
                file_path: args[2].clone(),
                ignore_case: env::var("CASE_INSENSITIVE").is_ok(),
            })
        }
    }
}
