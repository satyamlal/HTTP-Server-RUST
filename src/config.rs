use std::env;

pub struct Config {
    pub directory: String,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();
        let mut directory = String::from(".");

        while let Some(args) = args.next() {
            if args == "--directory" {
                directory = match args.next() {
                    Some(val) = val,
                    None => return Err("No value provided for --directory"),
                };
            }
        }
        Ok(config {directory} )
    }
}