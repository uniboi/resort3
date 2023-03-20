use sqparse::ast::{BinaryExpression, BinaryOperator};

use crate::rep::expressions::get_expression_rep;

use super::tokens::get_token;

pub fn get_binary_rep(p: &BinaryExpression, depth: usize) -> String {
    format!(
        "{} {} {}",
        get_expression_rep(&*p.left, depth),
        get_binary_operator_rep(&p.operator, depth),
        get_expression_rep(&*p.right, depth)
    )
}

fn get_binary_operator_rep(op: &BinaryOperator, depth: usize) -> String {
    match op {
        sqparse::ast::BinaryOperator::Assign(token) => get_token(token, "=", depth),
        sqparse::ast::BinaryOperator::AssignNewSlot(head, tail) => {
            format!("{}{}", get_token(head, "<", depth), get_token(tail, "-", depth))
        }
        sqparse::ast::BinaryOperator::AssignAdd(token) => get_token(token, "+=", depth),
        sqparse::ast::BinaryOperator::AssignSubtract(token) => get_token(token, "-=", depth),
        sqparse::ast::BinaryOperator::AssignMultiply(token) => get_token(token, "*=", depth),
        sqparse::ast::BinaryOperator::AssignDivide(token) => get_token(token, "/=", depth),
        sqparse::ast::BinaryOperator::AssignModulo(token) => get_token(token, "%=", depth),
        sqparse::ast::BinaryOperator::Add(token) => get_token(token, "+", depth),
        sqparse::ast::BinaryOperator::Subtract(token) => get_token(token, "-", depth),
        sqparse::ast::BinaryOperator::Multiply(token) => get_token(token, "*", depth),
        sqparse::ast::BinaryOperator::Divide(token) => get_token(token, "/", depth),
        sqparse::ast::BinaryOperator::Modulo(token) => get_token(token, "%", depth),
        sqparse::ast::BinaryOperator::Equal(token) => get_token(token, "==", depth),
        sqparse::ast::BinaryOperator::NotEqual(token) => get_token(token, "!=", depth),
        sqparse::ast::BinaryOperator::Less(token) => get_token(token, "<", depth),
        sqparse::ast::BinaryOperator::LessEqual(token) => get_token(token, "<=", depth),
        sqparse::ast::BinaryOperator::Greater(token) => get_token(token, ">", depth),
        sqparse::ast::BinaryOperator::GreaterEqual(token) => get_token(token, ">=", depth),
        sqparse::ast::BinaryOperator::ThreeWay(token) => get_token(token, "<=>", depth),
        sqparse::ast::BinaryOperator::LogicalAnd(token) => get_token(token, "&&", depth),
        sqparse::ast::BinaryOperator::LogicalOr(token) => get_token(token, "||", depth),
        sqparse::ast::BinaryOperator::BitwiseAnd(token) => get_token(token, "&", depth),
        sqparse::ast::BinaryOperator::BitwiseOr(token) => get_token(token, "|", depth),
        sqparse::ast::BinaryOperator::BitwiseXor(token) => get_token(token, "^", depth),
        sqparse::ast::BinaryOperator::ShiftLeft(head, tail) => {
            format!("{}{}", get_token(head, "<", depth), get_token(tail, "<", depth))
        }
        sqparse::ast::BinaryOperator::ShiftRight(head, tail) => {
            format!("{}{}", get_token(head, ">", depth), get_token(tail, ">", depth))
        }
        sqparse::ast::BinaryOperator::UnsignedShiftRight(head, body, tail) => format!(
            "{}{}{}",
            get_token(head, ">", depth),
            get_token(body, ">", depth),
            get_token(tail, ">", depth)
        ),
        sqparse::ast::BinaryOperator::In(token) => get_token(token, "in", depth),
        sqparse::ast::BinaryOperator::Instanceof(token) => get_token(token, "instanceof", depth),
    }
}
