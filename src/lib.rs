use std::fs;

#[derive(Debug)]
pub struct Config {
    machine: String,
    username: String,
    password: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 4 {
            return Err("Not enough arguments");
        }
        let machine = args[1].clone();
        let username = args[2].clone();
        let password = args[3].clone();
    
        Ok(Config { machine, username, password })
    }

    pub fn output(&self) -> String {
        format!("machine {} username {} password {}", self.machine, self.username, self.password)
    }

    pub fn write(&self) {
        if let Some(dir) = home::home_dir() {
            let file = format!("{}/.netrc", dir.display());
            fs::write(file, self.output()).expect("Unable to write file");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn output_result() {
        let output_string = "machine github.com username x-access-token password asdqwe123";
        let output_args : [String; 4] = 
            ["1".to_string(), 
            "github.com".to_string(), 
            "x-access-token".to_string(), 
            "asdqwe123".to_string()];
        let output_config = Config::new(&output_args);

        assert_eq!(output_string, output_config.unwrap().output());
    }
}