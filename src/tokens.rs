use sqparse::token::{Comment, Token, TokenLine};

pub fn get_token(token: &Token, p: &str) -> String {
	let pre_token_lines = get_pre_token_lines(token);
	let pre_token_comments = get_pre_token_comments(token);
	let post_token_lines = get_post_token_lines(token);
	format!("{pre_token_lines}{pre_token_comments}{p}{post_token_lines}")
}

fn get_post_token_lines(token: &Token) -> String {
    match &token.new_line {
        Some(line) => get_comments(&line.comments),
        None => String::new(),
    }
}

fn get_pre_token_lines(token: &Token) -> String {
    token.before_lines.iter().map(get_line_rep).collect()
}

fn get_line_rep(line: &TokenLine) -> String {
    format!("{}\n", get_comments(&line.comments))
}

fn get_pre_token_comments(token: &Token) -> String {
    if token.comments.len() == 0 {
        return String::new();
    }
    format!("{} ", get_comments(&token.comments))
}

fn get_comments(comments: &Vec<Comment>) -> String {
    comments
        .iter()
        .map(|comment| match comment {
            Comment::MultiLine(c) => get_multiline_comment_rep(c),
            sqparse::token::Comment::SingleLine(c) => get_single_comment_rep(c),
            _ => todo!(),
        })
        .collect::<Vec<_>>()
        .join(" ")
}

fn get_multiline_comment_rep(comment: &str) -> String {
    format!("/*{comment}*/")
}

fn get_single_comment_rep(comment: &str) -> String {
    format!("//{comment}")
}
