use sqparse::ast::IfStatement;

use crate::{
    get_statement_rep, parens_rep::get_parens_rep, utils::get_lead,
};

pub fn get_if_rep(stm: &IfStatement, depth: usize) -> String {
    format!(
        "if{}{}",
        get_parens_rep(&*stm.condition, depth),
        match &stm.ty {
            sqparse::ast::IfStatementType::NoElse { body } => get_statement_rep(&*body, depth),
            sqparse::ast::IfStatementType::Else {
                body,
                else_: _,
                else_body,
            } => format!("{}\n{}else {}", get_statement_rep(&body.ty, depth), get_lead(depth), get_statement_rep(&*else_body, depth)),
        }
    )
}
