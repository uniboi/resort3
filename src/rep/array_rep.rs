use sqparse::ast::Preprocessable;

use crate::rep::preprocessed::get_preprocessed_rep;
use crate::{
    get_config,
    rep::{expressions::get_expression_rep, tokens::get_token},
    utils::{
        get_lead, get_optional_padding, get_optional_seperator_rep,
        rep_includes_single_line_comment, trim_trailing_newline,
    },
};

pub fn get_array_rep(exp: &sqparse::ast::ArrayExpression, depth: usize) -> String {
    let padding = get_optional_padding(
        get_config()
            .lock()
            .unwrap()
            .array_oneliner_definition_padding,
    );
    let max_oneliner_items = get_config().lock().unwrap().array_oneliner_max;

    let oneliner_rep = format!("{padding}{}{padding}", get_array_oneliner_rep(exp, depth));

    let mut oneliner =
        exp.values.len() <= max_oneliner_items && !rep_includes_single_line_comment(&oneliner_rep);
    for v in &exp.values {
        match &v {
            Preprocessable::PREPROCESSED(_) => {
                oneliner = false;
                break;
            }
            Preprocessable::UNCONDITIONAL(_) => {}
        }
    }

    format!(
        "{}{}{}",
        get_token(exp.open, "[", depth),
        if exp.values.len() == 0 {
            match &exp.spread {
                Some(t) => get_token(t, "...", depth),
                None => String::new(),
            }
        } else if oneliner {
            oneliner_rep
        } else {
            format!(
                "\n{}{}\n{}",
                get_lead(depth + 1),
                get_array_multiliner_rep(exp, depth + 1),
                get_lead(depth)
            )
        },
        get_token(exp.close, "]", depth),
    )
}

fn get_array_oneliner_rep(exp: &sqparse::ast::ArrayExpression, depth: usize) -> String {
    let rep = exp
        .values
        .iter()
        .map(|v| match v {
            Preprocessable::PREPROCESSED(_) => panic!(), // this case is sorted out before
            Preprocessable::UNCONDITIONAL(v) => {
                format!(
                    "{}{}",
                    get_expression_rep(&*v.value, depth),
                    match &v.separator {
                        Some(sep) => get_token(sep, ",", depth),
                        None => String::new(),
                    }
                )
            }
        })
        .collect::<String>();
    format!(
        "{rep}{}",
        match exp.spread {
            Some(t) => get_token(t, "...", depth),
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
                let mut comma = get_optional_seperator_rep(&v.separator, depth);
                trim_trailing_newline(&mut comma);
                format!("{}{}", get_expression_rep(&*v.value, depth), comma)
            }
        })
        .collect::<Vec<_>>()
        .join(&format!("\n{}", get_lead(depth)));
    format!(
        "{rep}{}",
        match exp.spread {
            Some(t) => format!("\n{}{}", get_lead(depth), get_token(t, "...", depth)),
            None => String::new(),
        }
    )
}
