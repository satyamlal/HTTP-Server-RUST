use std::env;

pub struct Config {
    pub directory: String,
}

impl Config {
    pub fn build() -> Config {
        let args: Vec<String> = env::args::collect();

        let directory = if args.len() > 2 && args[1] == "--directory" {
            args[2].clone();
        } else {
            String::from(".")
        };
        Config{ directory }
    }
}