use crate::{get_expression_rep, tokens::get_token};

pub fn get_property_rep(exp: &sqparse::ast::PropertyExpression, depth: usize) -> String {
    format!(
        "{}{}{}",
        get_expression_rep(&*exp.base, depth),
        get_token(exp.dot, ".", depth),
        get_method_identifier_rep(&exp.property, depth)
    )
}

fn get_method_identifier_rep(exp: &sqparse::ast::MethodIdentifier, depth: usize) -> String {
    match exp {
        sqparse::ast::MethodIdentifier::Identifier(exp) => String::from(exp.value),
        sqparse::ast::MethodIdentifier::Constructor(token) => get_token(token, "constructor", depth),
    }
}
