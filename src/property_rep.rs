use crate::get_expression_rep;

pub fn get_property_rep(exp: &sqparse::ast::PropertyExpression) -> String {
    format!(
        "{}.{}",
        get_expression_rep(&*exp.base),
        get_method_identifier_rep(&exp.property)
    )
}

fn get_method_identifier_rep(exp: &sqparse::ast::MethodIdentifier) -> String {
    String::from(match exp {
        sqparse::ast::MethodIdentifier::Identifier(exp) => exp.value,
        sqparse::ast::MethodIdentifier::Constructor(_) => "constructor",
    })
}