use sqparse::ast::WhileStatement;

use crate::{parens_rep::get_parens_rep, get_expression_rep, get_statement_rep};

pub fn get_while_rep(stm: &WhileStatement, depth: usize) -> String {
    format!("while{}{}", get_parens_rep(&*stm.condition, depth), get_statement_rep(&*stm.body, depth))
}
