use sqparse::ast::{BlockStatement, StatementType};

use crate::{get_statement_rep, utils::get_lead};

pub fn get_block_rep(block: &BlockStatement, depth: usize) -> String {
    let inline_pre = get_lead(depth);
    let statements_pre = get_lead(depth + 1);
    let pre = format!("\n{inline_pre}");
    format!(
        "{pre}{{\n{}{}{pre}}}",
        statements_pre,
        block
            .statements
            .iter()
            .map(|statement| get_statement_rep(&statement.ty, depth + 1))
            .collect::<Vec<_>>()
            .join(&format!("\n{statements_pre}"))
    )
}

pub fn inset_statement_rep(stm: &StatementType, depth: usize) -> String {
    match &stm {
        StatementType::Block(_) => get_statement_rep(stm, depth),
        _ => format!("\n{}{}", get_lead(depth + 1), get_statement_rep(stm, depth + 1)),
    }
}
