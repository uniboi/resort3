mod array_rep;
mod binary_rep;
mod block_rep;
mod class_rep;
mod config;
mod constructor_res;
mod enum_rep;
mod expressions;
mod fix_rep;
mod for_rep;
mod foreach_rep;
mod function_rep;
mod global_rep;
mod if_rep;
mod literal_rep;
mod preprocessed;
mod property_rep;
mod statements;
mod struct_rep;
mod switch_rep;
mod table_rep;
mod tokens;
mod try_rep;
mod type_rep;
mod utils;
mod var_rep;
mod while_rep;
mod yields_rep;

use preprocessed::get_preprocessed_if_rep;
use sqparse::{parse, tokenize, Flavor};

use std::{borrow::BorrowMut, env, fs, sync::Mutex};

use crate::{
    config::Config, expressions::get_expression_rep, statements::get_full_statement_rep,
    utils::trim_trailing_newline,
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
