use sqparse::token::{Comment, Token, TokenLine};

use crate::utils::{get_lead, trim_trailing_newline};

pub fn get_token(token: &Token, p: &str, depth: usize) -> String {
    // let pre_token_lines = get_pre_token_lines(token, depth);
    let pre_token_lines = String::new();
    let pre_token_comments = get_pre_token_comments(token, depth);
    let post_token_lines = get_post_token_lines(token, depth);
    format!("{pre_token_lines}{pre_token_comments}{p}{post_token_lines}")
}

fn get_post_token_lines(token: &Token, depth: usize) -> String {
    match &token.new_line {
        Some(line) => {
            let mut comments = get_comments(&line.comments, depth);
            if comments.len() > 0 {
                comments = format!(" {comments}\n");
            }
            comments
        }
        None => String::new(),
    }
}

fn get_pre_token_lines(token: &Token, depth: usize) -> String {
    let lead = get_lead(depth);
    format!(
        "{}{}",
        token
            .before_lines
            .iter()
            .map(|line| get_line_rep(line, depth))
            // .map(|line| "test\n")
            .collect::<Vec<_>>()
            .join(&format!("{lead}")),
        if token.before_lines.len() > 0 {
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
    if token.comments.len() == 0 {
        return String::new();
    }
    format!("{} ", get_comments(&token.comments, depth))
}

fn get_comments(comments: &Vec<Comment>, depth: usize) -> String {
    comments
        .iter()
        .map(|comment| match comment {
            Comment::MultiLine(c) => get_multiline_comment_rep(c, depth),
            sqparse::token::Comment::SingleLine(c) => get_single_comment_rep(c, depth),
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
