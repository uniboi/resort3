mod rep;
mod config;
mod utils;

use config::Config;
use sqparse::{parse, tokenize, Flavor};

use std::{borrow::BorrowMut, env, fs, sync::Mutex};

use crate::{
    utils::trim_trailing_newline, rep::statements::get_full_statement_rep,
};

use std::sync::Once;

static mut CONFIG: Option<Mutex<Config>> = None;
static INIT: Once = Once::new();

fn main() {
    let args: Vec<String> = env::args().collect();
    let sript_path = args
        .get(1)
        .expect("no path to directory or file was provided");
    let config_path = args.get(2).expect("no configuration file was provided");

    load_config(config_path);

    println!("// Automatically generated from \"{}\"", sript_path);

    let source = fs::read_to_string(sript_path).expect("Failed reading file");

    let tokens = tokenize(&source, Flavor::SquirrelRespawn).unwrap();
    let ast = parse(&tokens, Flavor::SquirrelRespawn).unwrap();

    // println!("{ast:#?}")
    for statement in ast.statements {
        let mut stm = get_full_statement_rep(&statement, 0);
        trim_trailing_newline(&mut stm);
        print!("{}\n", stm)
    }
}

fn load_config<'a>(path: &String) {
    unsafe {
        INIT.call_once(|| {
            *CONFIG.borrow_mut() = Some(Mutex::new(Config::from_path(path).unwrap()));
        })
    };
}

fn get_config<'a>() -> &'a Mutex<Config> {
    unsafe { CONFIG.as_ref().unwrap() }
}

fn help() {
	println!("test")
}
