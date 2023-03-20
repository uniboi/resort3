use sqparse::ast::Expression;

use crate::{
    get_config,
    utils::{get_lead, get_optional_padding}, rep::{type_rep::get_typed_type_rep, preprocessed::get_preprocessed_if_rep},
};

use super::{tokens::get_token, literal_rep::{get_literal_rep, get_vector_rep}, property_rep::get_property_rep, fix_rep::{get_prefixed_expression_rep, get_postfixed_expression_rep}, table_rep::get_table_rep, class_rep::get_class_expression_rep, array_rep::get_array_rep, function_rep::{get_function_rep, get_call_rep}, binary_rep::get_binary_rep};

pub fn get_expression_rep(expression: &Expression, depth: usize) -> String {
    match expression {
        Expression::Parens(p) => format!(
            "{} {} {}",
            get_token(p.open, "(", depth),
            get_expression_rep(&*p.value, depth),
            get_token(p.close, ")", depth)
        ),
        Expression::Literal(p) => get_literal_rep(p, depth),
        Expression::Var(p) => get_token(p.name.token, p.name.value, depth),
        Expression::RootVar(p) => format!("::{}", p.name.value),
        Expression::Index(p) => format!(
            "{}{} {} {}",
            get_expression_rep(&*p.base, depth),
            get_token(p.open, "[", depth),
            get_expression_rep(&*p.index, depth),
            get_token(p.close, "]", depth),
        ),
        Expression::Property(p) => get_property_rep(p, depth),
        Expression::Ternary(p) => format!(
            "{} {} {} {} {}",
            get_expression_rep(&*p.condition, depth),
            get_token(p.question, "?", depth),
            get_expression_rep(&*p.true_value, depth),
            get_token(p.separator, ":", depth),
            get_expression_rep(&*p.false_value, depth)
        ),
        Expression::Binary(p) => get_binary_rep(p, depth),
        Expression::Prefix(p) => get_prefixed_expression_rep(p, depth),
        Expression::Postfix(p) => get_postfixed_expression_rep(p, depth),
        Expression::Comma(p) => format!(
            "{}{}",
            p.values
                .items
                .iter()
                .map(|(value, comma)| format!(
                    "{}{} ",
                    get_expression_rep(value, depth),
                    get_token(comma, ",", depth)
                ))
                .collect::<String>(),
            get_expression_rep(&*p.values.last_item, depth)
        ),
        Expression::Table(p) => get_table_rep(p, depth),
        Expression::Class(p) => get_class_expression_rep(p, depth),
        Expression::Array(p) => get_array_rep(p, depth),
        Expression::Function(p) => get_function_rep(p, depth),
        Expression::Call(p) => get_call_rep(p, depth),
        Expression::Delegate(p) => format!(
            "{} {} {} {}",
            get_token(p.delegate, "delegate", depth),
            get_expression_rep(&*p.parent, depth),
            get_token(p.colon, ":", depth),
            get_expression_rep(&*p.value, depth)
        ),
        Expression::Vector(p) => get_vector_rep(p, depth),
        Expression::Expect(p) => {
            let padding = get_optional_padding(get_config().lock().unwrap().expect_padding);
            format!(
                "{} {}{}{padding}{}{padding}{}",
                get_token(p.expect, "expect", depth),
                get_typed_type_rep(&p.ty, depth),
                get_token(p.open, "(", depth),
                get_expression_rep(&*p.value, depth),
                get_token(p.close, ")", depth),
            )
        }
        Expression::Lambda(_) => todo!(),
        Expression::Preprocessed(p) => {
            format!(
                "\n{}{}",
                get_lead(depth),
                get_preprocessed_if_rep(
                    &*p,
                    &|content, depth| format!(
                        "{}{}",
                        get_lead(depth + 1),
                        get_expression_rep(content, depth + 1)
                    ),
                    depth,
                )
            )
        }
    }
}
