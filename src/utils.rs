use sqparse::token::Token;

use crate::tokens::get_token;

pub fn get_lead(depth: usize) -> String {
	"\t".repeat(depth)
}

pub fn get_optional_seperator_rep(sep: &Option<&Token>, depth: usize) -> String {
    match &sep {
        Some(sep) => get_token(sep, ",", depth),
        None => String::from(","),
    }
}