use std::str::Split;

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

pub fn apply_lead_to_lines(lines: Split<&str>, depth: usize) -> String {
    let lead = get_lead(depth);
    lines
        .map(|line| format!("{lead}{line}"))
        .collect::<Vec<_>>()
        .join("\n")
}

pub fn clear_whitespace_lines(lines: Split<&str>, depth: usize) -> String {
    lines
        .map(|line| if line.trim().is_empty() { "" } else { line })
        .collect::<Vec<_>>()
        .join("\n")
}

pub fn trim_trailing_newline(s: &mut String) {
    if s.ends_with('\n') {
        s.pop();
    }
    if s.ends_with('\r') {
        s.pop();
    }
}
