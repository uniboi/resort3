use sqparse::ast::Expression;

use crate::get_expression_rep;

pub fn get_parens_rep(p: &Expression, depth: usize) -> String {
	format!("( {} )", get_expression_rep(&p, depth))
}