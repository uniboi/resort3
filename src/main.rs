mod cli;
mod config;
mod rep;
mod utils;

use crate::{rep::statements::get_full_statement_rep, utils::trim_trailing_newline};
use clap::Parser;
use cli::CliArgs;
use config::Config;
use sqparse::{parse, tokenize, Flavor};
use std::{
    fs::{self, File},
    io::{Error, Write},
    path::{Path, PathBuf},
    sync::Once,
};

static mut CONFIG: Option<Config> = None;
static INIT: Once = Once::new();

fn main() {
    let args = CliArgs::parse();

    let config_path = args.config;
    load_config(config_path);

    let path = Path::new(&args.input);
    let output_buf = match &args.output {
        Some(output) => output,
        None => &args.input,
    };

    let success = if path.is_file() {
        format_file(path, output_buf)
    } else {
        format_directory(&args.input, output_buf.clone())
    };

    success.unwrap();
}

fn format_directory(path: &PathBuf, mut output: PathBuf) -> Result<(), Error> {
    if path.is_dir() {
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let entry_path = entry.path();
            if entry_path.is_dir() {
                let p = entry_path.file_name().unwrap();
                output.push(p);
                format_directory(&entry_path, output.clone())?;
            } else {
                let extension = entry_path.extension();
                if matches!(extension, Some(_))
                    && extension.unwrap() != "nut"
                    && extension.unwrap() != "gnut"
                {
                    continue;
                }

                let mut output = output.clone();
                output.push(entry_path.file_name().unwrap());

                let mut location = output.clone();
                location.pop();

                std::fs::create_dir_all(&location)?;
                format_file(&entry_path, &output)?;
            }
        }
    }
    Ok(())
}

fn format_file(path: &Path, output: &Path) -> Result<(), Error> {
    let source = fs::read_to_string(path).expect("Failed reading file");
    let tokens = match tokenize(&source, Flavor::SquirrelRespawn) {
        Ok(tokens) => tokens,
        Err(err) => {
            eprintln!("{}", err.display(&source, path.to_str()));
            panic!("tokenizing {path:?} failed");
        }
    };
    let ast = match parse(&tokens, Flavor::SquirrelRespawn) {
        Ok(ast) => ast,
        Err(err) => {
            eprintln!("{}", err.display(&source, &tokens, path.to_str()));
            panic!("parsing {path:?} failed");
        }
    };

    let mut statements: Vec<String> = Vec::new();
    for statement in ast.statements {
        let mut stm = get_full_statement_rep(&statement, 0);
        trim_trailing_newline(&mut stm);
        statements.push(stm);
    }

    dump_output(output, statements)?;

    Ok(())
}

fn dump_output(path: &Path, statements: Vec<String>) -> Result<(), Error> {
    let mut file = File::create(path)?;
    writeln!(file, "{}", statements.join("\n").trim())?;

    Ok(())
}

fn load_config(path: Option<PathBuf>) {
    unsafe {
        INIT.call_once(|| match path {
            Some(path) => {
                CONFIG = Some(
                    Config::from_path(&path)
                        .unwrap_or_else(|_| panic!("failed loading configuration {:?}", &path)),
                )
            }
            None => CONFIG = Some(Config::new()),
        })
    }
}

fn get_config<'a>() -> &'a Config {
    unsafe { CONFIG.as_ref().unwrap() }
}
