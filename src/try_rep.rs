use sqparse::ast::ThrowStatement;

use crate::get_expression_rep;

pub fn throw_rep(p: &ThrowStatement, depth: usize) -> String {
	format!("throw {}", get_expression_rep(&*p.value, depth))
}