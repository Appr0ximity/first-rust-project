use std::{env::var, error::Error, fs, process::exit};

pub fn run(configuration: Config)-> Result<(), Box<dyn Error>>{
    let contents = fs::read_to_string(configuration.filename)?;

    if configuration.case_sensitive{
        for line in search(&configuration.query, &contents){
            println!("{}", line);
        }
    }else{
        for line in search_case_insenstitive(&configuration.query, &contents){
            println!("{}", line);
        }
    }
    Ok(())

}

pub struct Config{
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool
}

impl Config {
    pub fn new(args: &[String])-> Result<Config, &str>{

        if args.len() < 3{
            return Err("Not enough arguments!");
        } 

        let query = args[1].clone();
        let filename = args[2].clone();
        let case_sensitive = var("CASE_INSENSITIVE").is_err();
    
        Ok(Config {query, filename, case_sensitive})
    }
}

fn search<'a>(query: &str, contents: &'a str)-> Vec<&'a str>{
    let mut results: Vec<&str> = Vec::new();
    for line in contents.lines(){
        if line.contains(query){
            results.push(line);
        }
    }
    if results.len() == 0{
        println!("No lines including {} found", query);
        exit(1);
    }
    results
}

fn search_case_insenstitive<'a>(query: &str, contents: &'a str)-> Vec<&'a str>{
    let mut results: Vec<&str> = Vec::new();
    for line in contents.lines(){
        if line.to_lowercase().contains(query){
            results.push(line);
        }
    }
    if results.len() == 0{
        println!("No lines including {} found", query);
        exit(1);
    }
    results
}


#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn one_result(){
        let query: &str = "duct";
        let contents = "/
Rust:
safe, fast, productive.
Pick three.";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents))
    }

    #[test]
    fn case_insensitive(){
        let query: &str = "Rust";
        let contents = "/
Rust:
safe, fast, productive.
Pick three.
Trust me";

        assert_eq!(vec!["Rust:", "Trust me"], search_case_insenstitive(query, contents))
    }
}