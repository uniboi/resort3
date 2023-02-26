use sqparse::ast::{BinaryExpression, BinaryOperator};

use crate::get_expression_rep;

pub fn get_binary_rep(p: &BinaryExpression) -> String {
    format!(
        "{} {} {}",
        get_expression_rep(&*p.left),
        get_binary_operator_rep(&p.operator),
        get_expression_rep(&*p.right)
    )
}

fn get_binary_operator_rep(op: &BinaryOperator) -> String {
    String::from(match op {
        sqparse::ast::BinaryOperator::Assign(_) => "=",
        sqparse::ast::BinaryOperator::AssignNewSlot(_, _) => "<-",
        sqparse::ast::BinaryOperator::AssignAdd(_) => "+=",
        sqparse::ast::BinaryOperator::AssignSubtract(_) => "-=",
        sqparse::ast::BinaryOperator::AssignMultiply(_) => "*=",
        sqparse::ast::BinaryOperator::AssignDivide(_) => "/=",
        sqparse::ast::BinaryOperator::AssignModulo(_) => "%=",
        sqparse::ast::BinaryOperator::Add(_) => "+",
        sqparse::ast::BinaryOperator::Subtract(_) => "-",
        sqparse::ast::BinaryOperator::Multiply(_) => "*",
        sqparse::ast::BinaryOperator::Divide(_) => "/",
        sqparse::ast::BinaryOperator::Modulo(_) => "%",
        sqparse::ast::BinaryOperator::Equal(_) => "==",
        sqparse::ast::BinaryOperator::NotEqual(_) => "!=",
        sqparse::ast::BinaryOperator::Less(_) => "<",
        sqparse::ast::BinaryOperator::LessEqual(_) => "<=",
        sqparse::ast::BinaryOperator::Greater(_) => ">",
        sqparse::ast::BinaryOperator::GreaterEqual(_) => ">=",
        sqparse::ast::BinaryOperator::ThreeWay(_) => "<=>",
        sqparse::ast::BinaryOperator::LogicalAnd(_) => "&&",
        sqparse::ast::BinaryOperator::LogicalOr(_) => "||",
        sqparse::ast::BinaryOperator::BitwiseAnd(_) => "&",
        sqparse::ast::BinaryOperator::BitwiseOr(_) => "|",
        sqparse::ast::BinaryOperator::BitwiseXor(_) => "^",
        sqparse::ast::BinaryOperator::ShiftLeft(_, _) => "<<",
        sqparse::ast::BinaryOperator::ShiftRight(_, _) => ">>",
        sqparse::ast::BinaryOperator::UnsignedShiftRight(_, _, _) => ">>>",
        sqparse::ast::BinaryOperator::In(_) => "in",
        sqparse::ast::BinaryOperator::Instanceof(_) => "instanceof",
    })
}