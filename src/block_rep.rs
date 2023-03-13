use sqparse::ast::{BlockStatement, StatementType};

use crate::{get_statement_rep, tokens::get_token, utils::get_lead};

pub fn get_block_rep(block: &BlockStatement, depth: usize) -> String {
    let inline_pre = get_lead(depth);
    let statements_pre = get_lead(depth + 1);
    let pre = format!("\n{inline_pre}");
    format!(
        "{pre}{}\n{}{pre}{}",
        get_token(block.open, "{", depth),
        block
            .statements
            .iter()
            .map(|statement| format!(
                "{statements_pre}{}",
                get_statement_rep(&statement.ty, depth + 1)
            ))
            .collect::<Vec<_>>()
            .join("\n"),
        get_token(block.close, "}", depth),
    )
}

pub fn get_inset_statement_rep(stm: &StatementType, depth: usize) -> String {
    match &stm {
        StatementType::Block(_) => get_statement_rep(stm, depth),
        _ => format!(
            "\n{}{}",
            get_lead(depth + 1),
            get_statement_rep(stm, depth + 1)
        ),
    }
}
