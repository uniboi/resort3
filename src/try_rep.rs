use sqparse::ast::{ThrowStatement, TryCatchStatement};

use crate::{
    block_rep::get_inset_statement_rep, get_expression_rep, tokens::get_token, utils::get_lead,
};

pub fn throw_rep(p: &ThrowStatement, depth: usize) -> String {
    format!(
        "{} {}",
        get_token(p.throw, "throw"),
        get_expression_rep(&*p.value, depth)
    )
}

pub fn get_try_rep(p: &TryCatchStatement, depth: usize) -> String {
    format!(
        "{}{}\n{}{}{} {} {}{}",
        get_token(p.try_, "try"),
        get_inset_statement_rep(&p.body.ty, depth),
        get_lead(depth),
        get_token(p.catch, "catch"),
        get_token(p.open, "("),
        p.catch_name.value,
        get_token(p.close, ")"),
        get_inset_statement_rep(&p.catch_body, depth)
    )
}
