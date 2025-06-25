use std::io::{self, Write};

#[derive(Debug)]
pub struct Config {
    pub seed: u64,
    pub map_width: usize,
    pub map_height: usize,
    pub robots_count: usize,
}

impl Config {
    pub fn new() -> Config {
        let seed: u64 = Self::prompt_with_default("Seed", 42);
        let map_height: usize = Self::prompt_with_default("Map Height", 10);
        let map_width: usize = Self::prompt_with_default("Map Width", 10);
        let robots_count: usize = Self::prompt_with_default("Robots Count", 5);
        
        Config {
            map_height,
            map_width,
            robots_count,
            seed,
        }
    }

    pub fn prompt_with_default<T>(name: &str, default: T) -> T
    where
        T: std::str::FromStr + std::fmt::Display,
        <T as std::str::FromStr>::Err: std::fmt::Debug,
    {
        loop {
            print!("> {} [{}]: ", name, default);
            io::stdout().flush().unwrap();

            let mut input = String::new();
            if io::stdin().read_line(&mut input).is_err() {
                println!("⚠️ Error reading input. Using default.");
                return default;
            }

            let trimmed = input.trim();
            if trimmed.is_empty() {
                return default;
            }

            match trimmed.parse() {
                Ok(val) => return val,
                Err(_) => {
                    println!(
                        "❌ Invalid input. Please enter a valid number or press Enter for default."
                    );
                }
            }
        }
    }
}
