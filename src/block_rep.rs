use sqparse::ast::BlockStatement;

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
