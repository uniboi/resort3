use sqparse::ast::{
    Preprocessable, PreprocessorElseExpression, PreprocessorElseIfExpression,
    PreprocessorIfExpression,
};

use crate::{get_expression_rep, tokens::get_token, utils::get_lead};

pub fn get_preprocessed_if_rep<T, FnRep: Fn(&T, usize) -> String>(
    p: &PreprocessorIfExpression<T>,
    rep: &FnRep,
    depth: usize,
) -> String {
    let lead = get_lead(depth);
    let elseif_rep = match &p.elseif {
        Some(p) => get_preprocessed_elseif_rep(p, rep, depth),
        None => String::new(),
    };
    let else_rep = match &p.else_ {
        Some(p) => get_preprocessed_else_rep(p, rep, depth),
        None => String::new(),
    };
    format!(
        "{} {}\n{}{elseif_rep}{else_rep}\n{lead}{}",
        get_token(p.if_, "#if"),
        get_expression_rep(&*p.condition, depth),
        rep(&p.content, depth),
        get_token(p.endif, "#endif"),
    )
}

fn get_preprocessed_elseif_rep<T, FnRep: Fn(&T, usize) -> String>(
    p: &PreprocessorElseIfExpression<T>,
    rep: &FnRep,
    depth: usize,
) -> String {
    let lead = get_lead(depth);
    let elseif_rep = match &p.elseif {
        Some(p) => get_preprocessed_elseif_rep(p, rep, depth),
        None => String::new(),
    };
    format!(
        "\n{lead}{} {}\n{}{elseif_rep}",
        get_token(p.elseif_, "#elseif"),
        get_expression_rep(&*p.condition, depth),
        rep(&p.content, depth)
    )
}

fn get_preprocessed_else_rep<T, FnRep: Fn(&T, usize) -> String>(
    p: &PreprocessorElseExpression<T>,
    rep: FnRep,
    depth: usize,
) -> String {
    format!(
        "\n{}{}\n{}",
        get_lead(depth),
        get_token(p.else_, "#else"),
        rep(&p.content, depth)
    )
}

pub fn get_preprocessed_rep<T, FnRep: Fn(&T, usize) -> String>(
    p: &PreprocessorIfExpression<Vec<Preprocessable<T>>>,
    rep: &FnRep,
    depth: usize,
) -> String {
    let lead = get_lead(depth + 1);
    let f = |contents: &Vec<Preprocessable<T>>, depth| -> String {
        contents
            .iter()
            .map(|c| {
                format!(
                    "{lead}{}",
                    match c {
                        Preprocessable::PREPROCESSED(p) => get_preprocessed_rep(p, rep, depth + 1),
                        Preprocessable::UNCONDITIONAL(c) => rep(c, depth),
                    }
                )
            })
            .collect::<Vec<_>>()
            .join("\n")
    };
    get_preprocessed_if_rep(p, &f, depth)
}
