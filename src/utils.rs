use std::str::Split;

use sqparse::token::{Comment, Token};

use crate::rep::tokens::get_token;

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

pub fn clear_whitespace_lines(lines: Split<&str>, _depth: usize) -> String {
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

/// check if a list of tokens contains a single line comment after itself
pub fn tokens_include_single_line_comment(tokens: Vec<&Token>) -> bool {
    for token in tokens {
        if let Some(line) = &token.new_line {
            for comment in &line.comments {
                if let Comment::SingleLine(_) = comment {
                    return true;
                }
            }
        }
    }
    return false;
}

pub fn rep_includes_single_line_comment(rep: &String) -> bool {
    let comment_index = rep.find("//");
    if let None = comment_index {
        return false;
    }

    // TODO: handle // in multiline comments
    let sub = &rep[..comment_index.unwrap()];
    return sub.matches("\"").count() % 2 != 1; // if the number of quotes before `//` is odd, `//` is included in a string
}

pub fn rep_starts_with_comment(rep: &String) -> bool {
    let s = rep.trim();
    return matches!(s.find("//"), Some(0)) || matches!(s.find("/*"), Some(0));
}

pub fn get_optional_padding<'a>(padding: bool) -> &'a str {
	if padding {
		" "
	} else {
		""
	}
}
