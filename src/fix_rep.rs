use sqparse::ast::{PrefixExpression, PostfixExpression};

use crate::get_expression_rep;

pub fn get_prefixed_expression_rep(exp: &PrefixExpression, depth: usize) -> String {
	format!("{}{}", get_prefix_rep(&exp.operator), get_expression_rep(&*exp.value, depth))
}

fn get_prefix_rep(op: &sqparse::ast::PrefixOperator) -> String {
    String::from(match op {
        sqparse::ast::PrefixOperator::Negate(_) => "-",
        sqparse::ast::PrefixOperator::LogicalNot(_) => "!",
        sqparse::ast::PrefixOperator::BitwiseNot(_) => "~",
        sqparse::ast::PrefixOperator::Increment(_) => "++",
        sqparse::ast::PrefixOperator::Decrement(_) => "--",
		// These are keywords that require a space after
        sqparse::ast::PrefixOperator::Typeof(_) => "typeof ",
        sqparse::ast::PrefixOperator::Clone(_) => "clone ",
        sqparse::ast::PrefixOperator::Delete(_) => "delete ",
    })
}

pub fn get_postfixed_expression_rep(exp: &PostfixExpression, depth: usize) -> String {
	format!("{}{}", get_expression_rep(&*exp.value, depth), get_postfix_rep(&exp.operator))
}

fn get_postfix_rep(op: &sqparse::ast::PostfixOperator) -> String {
    String::from(match op {
        sqparse::ast::PostfixOperator::Increment(_) => "++",
        sqparse::ast::PostfixOperator::Decrement(_) => "--",
    })
}