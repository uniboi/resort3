use sqparse::ast::{Preprocessable, PreprocessorIfExpression};

use crate::{get_expression_rep, utils::get_lead};

pub fn get_preprocessable_rep<T, FnRep: Fn(&T, usize) -> String>(
    p: &PreprocessorIfExpression<T>,
    rep: FnRep,
    depth: usize,
) -> String {
    let lead = get_lead(depth);
    format!(
        "#if {}\n{}\n{lead}#endif",
        get_expression_rep(&*p.condition, depth),
        rep(&p.content, depth)
    )
}

pub fn pp_rep<T, FnRep: Fn(&T, usize) -> String>(
    p: &PreprocessorIfExpression<Vec<Preprocessable<T>>>,
	rep: FnRep,
    depth: usize,
) -> String {
    let lead = get_lead(depth + 1);
    get_preprocessable_rep(
        p,
        |contents: &Vec<Preprocessable<T>>, depth| -> String {
            contents
                .iter()
                .map(|c| match c {
                    Preprocessable::PREPROCESSED(p) => format!("{lead}{}", pp_rep(p, &rep, depth + 1)),
                    Preprocessable::UNCONDITIONAL(c) => {
                        format!("{lead}{}", rep(c, depth))
                    }
                })
                .collect::<Vec<_>>()
                .join("\n")
        },
        depth,
    )
}
