use sqparse::ast::{DoWhileStatement, StatementType, WhileStatement};

use crate::{
    get_config, get_full_statement_rep,
    rep::{
        block_rep::get_inset_statement_rep,
        expressions::get_expression_rep,
        statements::{append_semicolon, get_inline_statement_rep, get_statement_rep},
        tokens::get_token,
    },
    utils::{get_lead, get_optional_padding},
};

pub fn get_while_rep(stm: &WhileStatement, depth: usize) -> String {
    let gap = get_optional_padding(get_config().while_gap);
    let padding = get_optional_padding(get_config().while_padding);
    let inline = get_config().while_inline;

    format!(
        "{}{gap}{}{padding}{}{padding}{}{}",
        get_token(stm.while_, "while", depth),
        get_token(stm.open, "(", depth),
        get_expression_rep(&*stm.condition, depth),
        get_token(stm.close, ")", depth),
        format!(
            "{}",
            match &*stm.body {
                StatementType::Block(_) =>
                    get_inline_statement_rep(&*stm.body, depth, get_config().while_inline_block),
                _ => get_inline_statement_rep(&*stm.body, depth, inline),
            }
        )
    )
}

pub fn get_do_while_rep(stm: &DoWhileStatement, depth: usize) -> String {
    let padding = get_optional_padding(get_config().while_padding);
    let inline = get_config().do_while_inline;

    format!(
        "{}{}{}{}{padding}{}{padding}{}",
        get_token(stm.do_, "do", depth),
        match &stm.body.ty {
            StatementType::Block(_) => append_semicolon(
                &*stm.body,
                get_inline_statement_rep(&stm.body.ty, depth, get_config().do_while_inline_block),
                depth
            ),
            _ =>
                if inline {
                    format!(" {} ", get_full_statement_rep(&*stm.body, depth))
                } else {
                    format!(
                        "\n{}{}\n{}",
                        get_lead(depth + 1),
                        get_full_statement_rep(&*stm.body, depth),
                        get_lead(depth)
                    )
                },
        },
        get_token(stm.while_, "while", depth),
        get_token(stm.open, "(", depth),
        get_expression_rep(&*stm.condition, depth),
        get_token(stm.close, ")", depth),
    )
}
