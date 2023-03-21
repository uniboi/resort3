use sqparse::ast::{ReturnStatement, YieldStatement};

use crate::{
    get_config,
    rep::{expressions::get_expression_rep, tokens::get_token},
    utils::get_optional_padding,
};

pub fn get_delaythread_rep(expr: &sqparse::ast::DelayThreadStatement, depth: usize) -> String {
    let padding = get_optional_padding(get_config().delaythread_padding);
    let pre = get_optional_padding(get_config().delaythread_gap);

    format!(
        "{}{pre}{}{padding}{}{padding}{} {}",
        get_token(expr.delay_thread, "delaythread", depth),
        get_token(expr.open, "(", depth),
        get_expression_rep(&*expr.duration, depth),
        get_token(expr.close, ")", depth),
        get_expression_rep(&*expr.value, depth)
    )
}

pub fn get_return_rep(e: &ReturnStatement, depth: usize) -> String {
    format!(
        "{}{}",
        get_token(e.return_, "return", depth),
        match &e.value {
            Some(exp) => format!(" {}", get_expression_rep(exp, depth)),
            None => String::new(),
        }
    )
}

pub fn get_yield_rep(e: &YieldStatement, depth: usize) -> String {
    format!(
        "{}{}",
        get_token(e.yield_, "yield", depth),
        match &e.value {
            Some(exp) => format!(" {}", get_expression_rep(exp, depth)),
            None => String::new(),
        }
    )
}
