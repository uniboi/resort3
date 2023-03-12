use crate::{get_expression_rep, tokens::get_token};

pub fn get_property_rep(exp: &sqparse::ast::PropertyExpression, depth: usize) -> String {
    format!(
        "{}{}{}",
        get_expression_rep(&*exp.base, depth),
        get_token(exp.dot, "."),
        get_method_identifier_rep(&exp.property)
    )
}

fn get_method_identifier_rep(exp: &sqparse::ast::MethodIdentifier) -> String {
    match exp {
        sqparse::ast::MethodIdentifier::Identifier(exp) => String::from(exp.value),
        sqparse::ast::MethodIdentifier::Constructor(token) => get_token(token, "constructor"),
    }
}
