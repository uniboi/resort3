use sqparse::ast::{PostfixExpression, PrefixExpression};

use crate::{get_expression_rep, tokens::get_token};

pub fn get_prefixed_expression_rep(exp: &PrefixExpression, depth: usize) -> String {
    format!(
        "{}{}",
        get_prefix_rep(&exp.operator),
        get_expression_rep(&*exp.value, depth)
    )
}

fn get_prefix_rep(op: &sqparse::ast::PrefixOperator) -> String {
    match op {
        sqparse::ast::PrefixOperator::Negate(token) => get_token(token, "-"),
        sqparse::ast::PrefixOperator::LogicalNot(token) => get_token(token, "!"),
        sqparse::ast::PrefixOperator::BitwiseNot(token) => get_token(token, "~"),
        sqparse::ast::PrefixOperator::Increment(token) => get_token(token, "++"),
        sqparse::ast::PrefixOperator::Decrement(token) => get_token(token, "--"),
        // These are keywords that require a space after
        sqparse::ast::PrefixOperator::Typeof(token) => format!("{} ", get_token(token, "typeof")),
        sqparse::ast::PrefixOperator::Clone(token) => format!("{} ", get_token(token, "clone")),
        sqparse::ast::PrefixOperator::Delete(token) => format!("{} ", get_token(token, "delete")),
    }
}

pub fn get_postfixed_expression_rep(exp: &PostfixExpression, depth: usize) -> String {
    format!(
        "{}{}",
        get_expression_rep(&*exp.value, depth),
        get_postfix_rep(&exp.operator)
    )
}

fn get_postfix_rep(op: &sqparse::ast::PostfixOperator) -> String {
    match op {
        sqparse::ast::PostfixOperator::Increment(token) => get_token(token, "++"),
        sqparse::ast::PostfixOperator::Decrement(token) => get_token(token, "--"),
    }
}
