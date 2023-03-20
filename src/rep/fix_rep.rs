use sqparse::ast::{PostfixExpression, PrefixExpression};

use crate::rep::expressions::get_expression_rep;

use super::tokens::get_token;

pub fn get_prefixed_expression_rep(exp: &PrefixExpression, depth: usize) -> String {
    format!(
        "{}{}",
        get_prefix_rep(&exp.operator, depth),
        get_expression_rep(&*exp.value, depth)
    )
}

fn get_prefix_rep(op: &sqparse::ast::PrefixOperator, depth: usize) -> String {
    match op {
        sqparse::ast::PrefixOperator::Negate(token) => get_token(token, "-", depth),
        sqparse::ast::PrefixOperator::LogicalNot(token) => get_token(token, "!", depth),
        sqparse::ast::PrefixOperator::BitwiseNot(token) => get_token(token, "~", depth),
        sqparse::ast::PrefixOperator::Increment(token) => get_token(token, "++", depth),
        sqparse::ast::PrefixOperator::Decrement(token) => get_token(token, "--", depth),
        // These are keywords that require a space after
        sqparse::ast::PrefixOperator::Typeof(token) => {
            format!("{} ", get_token(token, "typeof", depth))
        }
        sqparse::ast::PrefixOperator::Clone(token) => {
            format!("{} ", get_token(token, "clone", depth))
        }
        sqparse::ast::PrefixOperator::Delete(token) => {
            format!("{} ", get_token(token, "delete", depth))
        }
    }
}

pub fn get_postfixed_expression_rep(exp: &PostfixExpression, depth: usize) -> String {
    format!(
        "{}{}",
        get_expression_rep(&*exp.value, depth),
        get_postfix_rep(&exp.operator, depth)
    )
}

fn get_postfix_rep(op: &sqparse::ast::PostfixOperator, depth: usize) -> String {
    match op {
        sqparse::ast::PostfixOperator::Increment(token) => get_token(token, "++", depth),
        sqparse::ast::PostfixOperator::Decrement(token) => get_token(token, "--", depth),
    }
}
