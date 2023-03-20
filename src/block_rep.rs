use sqparse::ast::{BlockStatement, StatementType};

use crate::{
    get_full_statement_rep,
    tokens::get_token,
    utils::{clear_whitespace_lines, get_lead, rep_starts_with_comment}, statements::get_statement_rep,
};

pub fn get_block_rep(block: &BlockStatement, depth: usize) -> String {
    let inline_pre = get_lead(depth);
    let statements_pre = get_lead(depth + 1);
    let pre = format!("\n{inline_pre}");

    let lines = block
        .statements
        .iter()
        .map(|statement| {
            let rep = get_full_statement_rep(&statement, depth + 1);
            let lines = rep.split("\n").collect::<Vec<_>>();
            let first_line = lines.get(0);

            // if rep.trim().find("//") == Some(0)
            if rep_starts_with_comment(&rep)
                || (matches!(lines.get(0), Some(_)) && first_line.unwrap().trim().is_empty())
            {
                rep
            } else {
                format!("{statements_pre}{rep}")
            }
        })
        .collect::<Vec<_>>();

    let opening = get_token(block.open, "{", depth);

    let rep = format!(
        "{}{}\n{}{pre}{}",
        if rep_starts_with_comment(&opening) {
            ""
        } else {
            // &pre
            ""
        },
        opening,
        lines.join("\n"),
        get_token(block.close, "}", depth),
    );
    clear_whitespace_lines(rep.split("\n"), depth)
}

pub fn get_inset_statement_rep(stm: &StatementType, depth: usize) -> String {
    match &stm {
        StatementType::Block(_) => {
            format!("\n{}{}", get_lead(depth), get_statement_rep(stm, depth))
        }
        _ => format!(
            "\n{}{}",
            get_lead(depth + 1),
            get_statement_rep(stm, depth + 1)
        ),
    }
}
