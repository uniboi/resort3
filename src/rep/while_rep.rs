use sqparse::ast::{DoWhileStatement, WhileStatement};

use crate::{
    get_full_statement_rep,
    rep::{expressions::get_expression_rep, statements::get_statement_rep, tokens::get_token},
};

pub fn get_while_rep(stm: &WhileStatement, depth: usize) -> String {
    format!(
        "{}{} {} {}{}",
        get_token(stm.while_, "while", depth),
        get_token(stm.open, "(", depth),
        get_expression_rep(&*stm.condition, depth),
        get_token(stm.close, ")", depth),
        get_statement_rep(&*stm.body, depth)
    )
}

pub fn get_do_while_rep(stm: &DoWhileStatement, depth: usize) -> String {
    format!(
        "{}{} {}{} {} {}",
        get_token(stm.do_, "do", depth),
        get_full_statement_rep(&*stm.body, depth),
        get_token(stm.while_, "while", depth),
        get_token(stm.open, "(", depth),
        get_expression_rep(&*stm.condition, depth),
        get_token(stm.close, ")", depth),
    )
}