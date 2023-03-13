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

pub fn trim_trailing_newline(s: &mut String) {
    if s.ends_with('\n') {
        s.pop();
        if s.ends_with('\r') {
            s.pop();
        }
    }
}
