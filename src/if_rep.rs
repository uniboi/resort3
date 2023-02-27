use sqparse::ast::{BlockStatement, IfStatement, Statement, StatementType};

use crate::{
    block_rep::get_block_rep, get_statement_rep, parens_rep::get_parens_rep, utils::get_lead,
};

pub fn get_if_rep(stm: &IfStatement, depth: usize) -> String {
    format!(
        "if{}{}",
        get_parens_rep(&*stm.condition, depth),
        match &stm.ty {
            sqparse::ast::IfStatementType::NoElse { body } => get_if_body_rep(&*body, depth),
            sqparse::ast::IfStatementType::Else {
                body,
                else_: _,
                else_body,
            } => format!(
                "{}\n{}else{}",
                get_if_body_rep(&body.ty, depth),
                get_lead(depth),
                get_if_body_rep(&*else_body, depth)
            ),
        }
    )
}

fn get_if_body_rep(stm: &StatementType, depth: usize) -> String {
    match &stm {
		StatementType::If(p) => format!(" {}", get_if_rep(p, depth)),
        StatementType::Block(p) => get_block_rep(p, depth),
        p => format!("\n{}{}", get_lead(depth + 1), get_statement_rep(p, depth + 1)),
    }
}
