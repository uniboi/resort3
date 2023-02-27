use sqparse::ast::{WhileStatement, DoWhileStatement};

use crate::{parens_rep::get_parens_rep, get_statement_rep};

pub fn get_while_rep(stm: &WhileStatement, depth: usize) -> String {
    format!("while{}{}", get_parens_rep(&*stm.condition, depth), get_statement_rep(&*stm.body, depth))
}

pub fn get_do_while_rep(stm: &DoWhileStatement, depth: usize) -> String {
	format!("do{} while{}", get_statement_rep(&stm.body.ty, depth), get_parens_rep(&*stm.condition, depth))
}
