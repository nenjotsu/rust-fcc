use minigrep_refactor::Config;
use std::env;
use std::process;

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    // index 0 is a binary path

    println!("Searching for {}", config.query);
    println!("in file {}", config.filename);
    // println!("{:?}", args);

    if let Err(e) = minigrep_refactor::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
