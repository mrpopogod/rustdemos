use std::env;
use std::error::Error;
use std::fs;
use std::io;

// trait allows us to have an interface-like way for getting data from a file that our tests can then provide a unit test
// friendly implemetnation of
trait ProvideData {
    fn get_data(&self) -> io::Result<String>;
}

// Data coming from a file on disk
struct FileData(String);

impl ProvideData for FileData {
    fn get_data(&self) -> io::Result<String> {
        fs::read_to_string(&self.0)
    }
}

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new<T>(mut args: T) -> Result<Config, &'static str>
    where
        T: Iterator<Item = String>, // In order for tests to work, we make this definition a bit more generic
    {
        args.next();

        // iterating over args instead of constructing a vector in main and processinjg it means we get to avoid clones;
        // instead we take ownership of the strings through the iterator.  That also means the Errs are now guaranteed
        // to have a static lifetime for their contents
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file name"),
        };

        let case_sensitive = env::var("MINIGREP_CASE_INSENSITIVE").is_err(); // note; if we ever use this in tests
                                                                             // we msut be careful, as setting env variables
                                                                             // are for the duration of the process and
                                                                             // rust tests are multithreaded by default

        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let data = FileData(config.filename.clone()); // initialize our file read before passing to actual function
                                                  // need to clone here or else we take ownership of filename away
                                                  // from config
    _run(&config, &data)
}

fn _run(config: &Config, data_provider: &dyn ProvideData) -> Result<(), Box<dyn Error>> {
    let contents = data_provider.get_data()?; // ? operator automatically returns Error if that is returned+

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
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

    // Data coming from a static resource
    struct StaticData(&'static str);

    impl ProvideData for StaticData {
        fn get_data(&self) -> io::Result<String> {
            Ok(self.0.to_string())
        }
    }

    struct ErrorData();

    impl ProvideData for ErrorData {
        fn get_data(&self) -> io::Result<String> {
            Err(io::Error::from(io::ErrorKind::NotFound))
        }
    }

    #[test]
    fn config_accepts_three_args() {
        let good_args = good_args();
        let args = good_args.iter().map(|s| s.to_string());
        let result = Config::new(args);
        assert!(
            result.is_ok(),
            "Config didn't successfully new, error was {:?}",
            result.err()
        );
    }

    #[test]
    fn config_requires_three_args() {
        let bad_args = vec!["first".to_string()];
        let args = bad_args.iter().map(|s| s.to_string());
        let result = Config::new(args);
        assert!(result.is_err(), "Config didn't complain when newed");
    }

    #[test]
    fn run_happy_with_good_data() {
        let data = StaticData("My file contents");
        let config = good_config();
        let result = _run(&config, &data);
        assert!(
            result.is_ok(),
            "_run failed when trying to process good input, error was {:?}",
            result.err()
        );
    }

    #[test]
    fn run_surfaces_error_with_bad_data() {
        let data = ErrorData();
        let config = good_config();
        let result = _run(&config, &data);
        assert!(
            result.is_err(),
            "_run didn't surface an error with a failed file load"
        );
    }

    #[test]
    fn case_sensitive_one_result() {
        let query = "duct";
        // multiline string literal; the \ as the last character removes the newline on that line plus leading whitespace
        // on the next line; in this case since we do want the newlines we need to put them at the start of the source line
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive_two_results() {
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

    fn good_args() -> Vec<String> {
        vec!["first".to_string(), "second".into(), "third".into()]
    }

    fn good_config() -> Config {
        let good_args = good_args(); // need to let to capture the returned good_args so it maintains its life through
                                     // the end of this function; if we try to immediately chain it with iter then
                                     // it will go out of scope at the end of the line (as it's temporary) and then the
                                     // variable containing the chained iterator mutations lost what it was holding
                                     // and things go wrong
        let args = good_args.iter().map(|s| s.to_string()); // iter returns an option so you know when iteration stops
                                                            // thus we need to map to extract the string inside
        Config::new(args).unwrap()
    }
}
