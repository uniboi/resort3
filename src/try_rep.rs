use sqparse::ast::{ThrowStatement, TryCatchStatement};

use crate::{block_rep::get_inset_statement_rep, get_expression_rep, utils::get_lead};

pub fn throw_rep(p: &ThrowStatement, depth: usize) -> String {
    format!("throw {}", get_expression_rep(&*p.value, depth))
}

pub fn get_try_rep(p: &TryCatchStatement, depth: usize) -> String {
    format!(
        "try{}\n{}catch( {} ){}",
        get_inset_statement_rep(&p.body.ty, depth),
        get_lead(depth),
        p.catch_name.value,
        get_inset_statement_rep(&p.catch_body, depth)
    )
}
