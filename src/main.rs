use std::{env, process::exit};

use minigrep::{run, Config};

fn main() {
    let args: Vec<String> = env::args().collect();

    let configuration = Config::new(&args).unwrap_or_else(|err|{
        println!("Problem parsing arguments: {}", err);
        exit(1)
    });

    println!("Searching for {}\nIn file name {}", configuration.query, configuration.filename);

    if let Err(e) = run(configuration){
        println!("Application Error {}", e);
        exit(1);
    }
}