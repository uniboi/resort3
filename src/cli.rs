use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct CliArgs {
    /// location of a directory / file to format
    #[arg(short, long)]
    pub input: PathBuf,
    /// location of the config file used
    #[arg(short, long)]
    pub config: Option<PathBuf>,
	#[arg(short, long)]
	pub output: Option<PathBuf>,
}
