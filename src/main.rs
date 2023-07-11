use std::{env, fs, process};
use std::error::Error;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err|{
        println!("Проблема при разборе аргументов: {}", err);
        process::exit(1);
    });

    println!("Поиск: {}", config.query);
    println!("В файле: {}", config.filename);

    if let Err(e) = run(config) {
        println!("Ошибка в приложении: {}", e);
        process::exit(1);
    };

    fn run(config: Config) -> Result<(), Box<dyn Error>>{
        let contents = fs::read_to_string(config.filename)?;
        println!("С текстом: \n{}", contents);
        Ok(())
    }

}
struct Config {
    query: String,
    filename: String,
}
impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Недостаточно аргументов");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        Ok(Config{query,filename})
    }
}
