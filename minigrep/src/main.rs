use std::env;

use std::process;

use minigrep::Config;

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {  // if let instead of unwrap_or_else because we don't care about the Ok value
        eprintln!("Application error: {}", e);

        process::exit(1);
    }
}
