use sqparse::ast::{DoWhileStatement, StatementType, WhileStatement};

use crate::{
    get_config, get_full_statement_rep,
    rep::{expressions::get_expression_rep, statements::get_statement_rep, tokens::get_token},
    utils::{get_lead, get_optional_padding},
};

pub fn get_while_rep(stm: &WhileStatement, depth: usize) -> String {
    let gap = get_optional_padding(get_config().lock().unwrap().while_gap);
    let padding = get_optional_padding(get_config().lock().unwrap().while_padding);

    format!(
        "{}{gap}{}{padding}{}{padding}{}{}",
        get_token(stm.while_, "while", depth),
        get_token(stm.open, "(", depth),
        get_expression_rep(&*stm.condition, depth),
        get_token(stm.close, ")", depth),
        format!(
            "\n{}",
            match &*stm.body {
                StatementType::Block(_) => format!(
                    "{}{}",
                    get_lead(depth),
                    get_statement_rep(&*stm.body, depth)
                ),
                _ => format!(
                    "{}{}",
                    get_lead(depth + 1),
                    get_statement_rep(&*stm.body, depth)
                ),
            }
        )
    )
}

pub fn get_do_while_rep(stm: &DoWhileStatement, depth: usize) -> String {
    let padding = get_optional_padding(get_config().lock().unwrap().while_padding);

    format!(
        "{}{}{}{}{padding}{}{padding}{}",
        get_token(stm.do_, "do", depth),
        format!(
            "\n{}",
            match &stm.body.ty {
                StatementType::Block(_) => format!(
                    "{}{} ",
                    get_lead(depth),
                    get_full_statement_rep(&*stm.body, depth)
                ),
                _ => format!(
                    "{}{}\n{}",
                    get_lead(depth + 1),
                    get_full_statement_rep(&*stm.body, depth),
                    get_lead(depth)
                ),
            }
        ),
        get_token(stm.while_, "while", depth),
        get_token(stm.open, "(", depth),
        get_expression_rep(&*stm.condition, depth),
        get_token(stm.close, ")", depth),
    )
}
