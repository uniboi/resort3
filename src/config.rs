use std::error::Error;
use std::fs::File;
use std::io::BufReader;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub semicolons: bool,
	pub delaythread_padding: bool,
	pub delaythread_gap: bool,
}

impl Config {
    pub fn from_path(path: &String) -> Result<Self, Box<dyn Error>> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);

        let p: Self = serde_json::from_reader(reader)?;

        Ok(p)
    }
}
