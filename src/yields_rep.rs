use sqparse::ast::{ReturnStatement, YieldStatement};

use crate::get_expression_rep;

pub fn get_delaythread_rep(expr: &sqparse::ast::DelayThreadStatement) -> String {
    let padding = " "; // TODO: read from config
    let pre = ""; // TODO: read from config
    format!(
        "delaythread{pre}({padding}{}{padding}) {}",
        get_expression_rep(&*expr.duration),
        get_expression_rep(&*expr.value)
    )
}

pub fn get_return_rep(e: &ReturnStatement) -> String {
    format!(
        "return{}",
        match &e.value {
            Some(exp) => format!(" {}", get_expression_rep(exp)),
            None => String::new(),
        }
    )
}

pub fn get_yield_rep(e: &YieldStatement) -> String {
    format!(
        "yield{}",
        match &e.value {
            Some(exp) => format!(" {}", get_expression_rep(exp)),
            None => String::new(),
        }
    )
}
