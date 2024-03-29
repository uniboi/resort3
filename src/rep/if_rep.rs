use sqparse::ast::{IfStatement, StatementType};

use crate::{
    get_config,
    rep::{expressions::get_expression_rep, tokens::get_token},
    utils::{get_lead, get_optional_padding},
};

use super::{block_rep::get_block_rep, statements::get_statement_rep};

// TODO: body semicolon token gets eaten
pub fn get_if_rep(stm: &IfStatement, depth: usize) -> String {
    let gap = get_optional_padding(get_config().if_gap);
    let padding = get_optional_padding(get_config().if_padding);
    let lead = get_lead(depth);
    format!(
        "{}{gap}{}{padding}{}{padding}{}{}",
        get_token(stm.if_, "if", depth),
        get_token(stm.open, "(", depth),
        get_expression_rep(&stm.condition, depth),
        get_token(stm.close, ")", depth),
        match &stm.ty {
            sqparse::ast::IfStatementType::NoElse { body } => get_if_body_rep(body, depth),
            sqparse::ast::IfStatementType::Else {
                body,
                else_,
                else_body,
            } => format!(
                "{}\n{lead}{}{}",
                get_if_body_rep(&body.ty, depth),
                get_token(else_, "else", depth),
                match **else_body {
                    StatementType::If(_) => get_if_body_rep(else_body, depth),
                    _ => get_if_body_rep(else_body, depth),
                },
            ),
            sqparse::ast::IfStatementType::NoElseTailless => String::new(),
            sqparse::ast::IfStatementType::ElseTailless { body, else_ } => format!(
                "{}\n{lead}{}",
                get_if_body_rep(&body.ty, depth),
                get_token(else_, "else", depth)
            ),
        }
    )
}

fn get_if_body_rep(stm: &StatementType, depth: usize) -> String {
    match &stm {
        StatementType::If(p) => format!(" {}", get_if_rep(p, depth)),
        StatementType::Block(p) => {
            if get_config().if_inline_block {
                format!(" {}", get_block_rep(p, depth))
            } else {
                format!("\n{}{}", get_lead(depth), get_block_rep(p, depth))
            }
        }
        p => format!(
            "{}{}",
            if get_config().if_inline {
                String::from(" ")
            } else {
                format!("\n{}", get_lead(depth + 1))
            },
            get_statement_rep(p, depth + 1)
        ),
    }
}
