use sqparse::ast::{IfStatement, StatementType};

use crate::{
    block_rep::get_block_rep, get_statement_rep, parens_rep::get_parens_rep, tokens::get_token,
    utils::get_lead,
};

pub fn get_if_rep(stm: &IfStatement, depth: usize) -> String {
    format!(
        "{}{}{}",
        get_token(stm.if_, "if"),
        get_parens_rep(&*stm.condition, depth),
        match &stm.ty {
            sqparse::ast::IfStatementType::NoElse { body } => get_if_body_rep(&*body, depth),
            sqparse::ast::IfStatementType::Else {
                body,
                else_,
                else_body,
            } => format!(
                "{}\n{}{}{}",
                get_if_body_rep(&body.ty, depth),
                get_lead(depth),
                get_token(else_, "else"),
                get_if_body_rep(&*else_body, depth)
            ),
        }
    )
}

fn get_if_body_rep(stm: &StatementType, depth: usize) -> String {
    match &stm {
        StatementType::If(p) => format!(" {}", get_if_rep(p, depth)),
        StatementType::Block(p) => get_block_rep(p, depth),
        p => format!(
            "\n{}{}",
            get_lead(depth + 1),
            get_statement_rep(p, depth + 1)
        ),
    }
}
