use sqparse::ast::Preprocessable;

use crate::{get_expression_rep, preprocessed::get_preprocessed_rep, utils::get_lead};

pub fn get_array_rep(exp: &sqparse::ast::ArrayExpression, depth: usize) -> String {
    let padding = " "; // TODO: read from config
    let max_oneliner_items = 5; // TODO: read from config

    let mut oneliner = exp.values.len() <= max_oneliner_items;
    for v in &exp.values {
        if matches!(v, Preprocessable::PREPROCESSED(_)) {
            oneliner = false;
            break;
        }
    }

    format!(
        "[{}]",
        if exp.values.len() == 0 {
            match &exp.spread {
                Some(_) => format!(" ... "),
                None => String::new(),
            }
        } else if oneliner {
            format!("{padding}{}{padding}", get_array_oneliner_rep(exp, depth))
        } else {
            format!(
                "\n{}{}\n{}",
                get_lead(depth + 1),
                get_array_multiliner_rep(exp, depth + 1),
                get_lead(depth)
            )
        }
    )
}

fn get_array_oneliner_rep(exp: &sqparse::ast::ArrayExpression, depth: usize) -> String {
    let spread = "...";
    let rep = exp
        .values
        .iter()
        .map(|v| match v {
            Preprocessable::PREPROCESSED(_) => panic!(), // this case is sorted out before
            Preprocessable::UNCONDITIONAL(v) => get_expression_rep(&*v.value, depth),
        })
        .collect::<Vec<_>>()
        .join(", ");
    format!(
        "{rep}{}",
        match exp.spread {
            Some(_) =>
                if exp.values.len() > 0 {
                    format!(", {spread}")
                } else {
                    format!("{spread}")
                },
            None => String::new(),
        }
    )
}

fn get_array_multiliner_rep(exp: &sqparse::ast::ArrayExpression, depth: usize) -> String {
    let rep = exp
        .values
        .iter()
        .map(|v| match v {
            Preprocessable::PREPROCESSED(v) => {
                get_preprocessed_rep(&*v, &|v, depth| get_expression_rep(&*v.value, depth), depth)
            }
            Preprocessable::UNCONDITIONAL(v) => {
                format!("{},", get_expression_rep(&*v.value, depth))
            }
        })
        .collect::<Vec<_>>()
        .join(&format!("\n{}", get_lead(depth)));
    format!(
        "{rep}{}",
        match exp.spread {
            Some(_) => format!(",\n{}...", get_lead(depth)),
            None => String::new(),
        }
    )
}
