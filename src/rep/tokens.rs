use sqparse::token::{Comment, Token, TokenLine};

use crate::utils::{get_lead, trim_trailing_newline};

pub fn get_token(token: &Token, p: &str, depth: usize) -> String {
    let pre_token_lines = get_pre_token_lines(token, depth);
    let pre_token_comments = get_pre_token_comments(token, depth);
    let post_token_lines = get_post_token_lines(token, depth);
    format!("{pre_token_lines}{pre_token_comments}{p}{post_token_lines}")
}

pub fn get_headless_token(token: &Token, p: &str, depth: usize) -> String {
    let pre_token_lines = get_pre_token_comment_lines(token, depth);
    let pre_token_comments = get_pre_token_comments(token, depth);
    let post_token_lines = get_post_token_lines(token, depth);
    format!("{pre_token_lines}{pre_token_comments}{p}{post_token_lines}")
}

fn get_post_token_lines(token: &Token, depth: usize) -> String {
    match &token.new_line {
        Some(line) => {
            let mut comments = get_comments(&line.comments, depth);
            if !comments.is_empty() {
                comments = format!(" {comments}");
            }
            comments
        }
        None => String::new(),
    }
}

pub fn get_pre_token_comment_lines(token: &Token, depth: usize) -> String {
    let lines = token
        .before_lines
        .iter()
        .filter(|line| !line.comments.is_empty())
        .collect::<Vec<_>>()
        .iter()
        .map(|line| format!("{}{}", get_lead(depth), get_line_rep(line, depth)))
        .collect::<Vec<_>>();
    format!(
        "{}{}",
        lines.join(""),
        if !lines.is_empty() {
            get_lead(depth)
        } else {
            String::new()
        }
    )
}

pub fn get_pre_token_lines(token: &Token, depth: usize) -> String {
    let lead = get_lead(depth);
    let mut prev_line_empty = false;
    let trim_empty_lines = true; // TODO: untangle this spaghetti
    format!(
        "{}{}",
        token
            .before_lines
            .iter()
            .map(|line| {
                let rep = get_line_rep(line, depth);
                if rep.trim().is_empty() {
                    if prev_line_empty && trim_empty_lines {
                        return String::new();
                    }
                    prev_line_empty = true;
                } else {
                    prev_line_empty = false;
                }
                let final_rep = format!("{}{rep}", if rep.trim().is_empty() { "" } else { &lead });
                final_rep
            })
            .collect::<String>(),
        if !token.before_lines.is_empty() {
            lead
        } else {
            String::new()
        }
    )
}

fn get_line_rep(line: &TokenLine, depth: usize) -> String {
    format!("{}\n", get_comments(&line.comments, depth))
}

fn get_pre_token_comments(token: &Token, depth: usize) -> String {
    if token.comments.is_empty() {
        return String::new();
    }
    format!("{} ", get_comments(&token.comments, depth))
}

fn get_comments(comments: &[Comment], depth: usize) -> String {
    comments
        .iter()
        .map(|comment| match comment {
            Comment::MultiLine(c) => get_multiline_comment_rep(c, depth),
            Comment::SingleLine(c) => get_single_comment_rep(c, depth),
            _ => todo!(),
        })
        .collect::<Vec<_>>()
        .join(" ")
}

fn get_multiline_comment_rep(comment: &str, _depth: usize) -> String {
    format!("/*{comment}*/")
}

fn get_single_comment_rep(comment: &str, _depth: usize) -> String {
    let mut c = String::from(comment);
    trim_trailing_newline(&mut c);
    format!("//{c}")
}
