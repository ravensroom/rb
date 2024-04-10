pub mod minigrep {
    use std::env;
    use std::fs;
    use std::process;

    pub fn run() {
        let config = Config::build(env::args()).unwrap_or_else(|err| {
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
        contents
            .lines()
            .filter(|line| line.contains(query))
            .collect()
    }

    fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
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
        fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
            args.next();

            let query = match args.next() {
                Some(arg) => arg,
                None => return Err("Didn't get a query string"),
            };

            let file_path = match args.next() {
                Some(arg) => arg,
                None => return Err("Didn't get a file path"),
            };

            let ignore_case = env::var("IGNORE_CASE").is_ok();

            Ok(Config {
                query,
                file_path,
                ignore_case,
            })
        }
    }
}
