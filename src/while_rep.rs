use sqparse::ast::{DoWhileStatement, WhileStatement};

use crate::{get_expression_rep, get_statement_rep, tokens::get_token};

pub fn get_while_rep(stm: &WhileStatement, depth: usize) -> String {
    format!(
        "{}{} {} {}{}",
        get_token(stm.while_, "while"),
        get_token(stm.open, "("),
        get_expression_rep(&*stm.condition, depth),
        get_token(stm.close, ")"),
        get_statement_rep(&*stm.body, depth)
    )
}

pub fn get_do_while_rep(stm: &DoWhileStatement, depth: usize) -> String {
    format!(
        "{}{} {}{} {} {}",
        get_token(stm.do_, "do"),
        get_statement_rep(&stm.body.ty, depth),
        get_token(stm.while_, "while"),
        get_token(stm.open, "("),
        get_expression_rep(&*stm.condition, depth),
        get_token(stm.close, ")"),
    )
}
