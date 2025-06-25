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
        let seed: u64 = 15;
        let map_height= 20;
        let map_width = 20;
        let robots_count = 5;

        Config {
            map_height,
            map_width,
            robots_count,
            seed,
        }
    }
}