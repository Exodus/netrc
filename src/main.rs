use std::env;
use std::process;

use netrc::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Error: Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("{:?}", config);
    println!("{}",config.output());

    config.write()
}

