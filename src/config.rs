use std::error::Error;
use std::fs::File;
use std::io::BufReader;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Config {
    /// Add semicolons to statements
    pub semicolons: bool,
    /// add padding to the brackets of `delaythread`
    pub delaythread_padding: bool,
    /// add a gap between `delaythread` and the timer brackets
    pub delaythread_gap: bool,
    /// defines the number of items in an array definition that will be represented in a single line defintion
    pub array_oneliner_max: usize,
    /// add padding in oneliner array definitions
    pub array_oneliner_definition_padding: bool,
    /// add padding in the var brackets
    pub expect_padding: bool,
    /// trim the leading 0 of a float and start at the dot, if possible
    pub trim_float: bool,
	/// add padding between between the open and close tokens of a vector
	pub vector_padding: bool,
	/// defines the number of args a functionref takes before it will be forced to be multiline
	pub functionref_oneliner_args_max: usize,
	/// add padding to generics holding non-generic types. e.g. `table<string, int>`
	pub non_generic_type_padding: bool,
	/// defines the number of items in a table definition that will be represented in a single line definition
	pub table_oneliner_max: usize,
}

impl Config {
    pub fn from_path(path: &String) -> Result<Self, Box<dyn Error>> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);

        let p: Self = serde_json::from_reader(reader)?;

        Ok(p)
    }
}
