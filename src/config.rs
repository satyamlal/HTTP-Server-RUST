pub struct Config {
    pub directory: String, 
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next(); 
        let mut directory = String::from(".");
        
        while let Some(arg) = args.next() { 
            if arg == "--directory" {
                directory = match args.next() {
                    Some(val) => val,
                    None => return Err("No value provided for --directory"),
                };
            }
        }
        Ok(Config { directory }) 
    }
}