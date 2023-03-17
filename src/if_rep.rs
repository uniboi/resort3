use sqparse::ast::{IfStatement, StatementType};

use crate::{
    block_rep::get_block_rep, get_expression_rep, get_statement_rep, tokens::get_token,
    utils::get_lead,
};

pub fn get_if_rep(stm: &IfStatement, depth: usize) -> String {
    let lead = get_lead(depth);
    format!(
        "{}{} {} {}{}",
        get_token(stm.if_, "if", depth),
        get_token(stm.open, "(", depth),
        get_expression_rep(&*stm.condition, depth),
        get_token(stm.close, ")", depth),
        match &stm.ty {
            sqparse::ast::IfStatementType::NoElse { body } => get_if_body_rep(&*body, depth),
            sqparse::ast::IfStatementType::Else {
                body,
                else_,
                else_body,
            } => format!(
                "\n{lead}{}\n{lead}{}{}",
                get_if_body_rep(&body.ty, depth), // TODO: comments after semicolons get eaten
                get_token(else_, "else", depth),
                match &**else_body {
                    StatementType::If(_) => get_if_body_rep(&*else_body, depth),
                    _ => format!("\n{lead}{}", get_if_body_rep(&*else_body, depth)),
                },
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
