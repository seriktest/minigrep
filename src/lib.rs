use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub filename: String,
}
impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Недостаточно аргументов");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        Ok(Config { query, filename })
    }
}
pub fn run(config: Config) -> Result<(), Box<dyn Error>>{
    let contents = fs::read_to_string(config.filename)?;
    println!("С текстом: \n{}", contents);
    Ok(())
}

#[cfg(test)]
mod tests {

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
        \
        Rust:\
        safe, fast, productive.\
        Pick three.";

        assert_eq!(vec!["save, fast, productive."], search(query, contents));
    }
    pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
        let mut result = Vec::new();

        for line in contents.lines() {
            if line.contains(query) {
                result.push(line);
            }
        }
        result
    }
}