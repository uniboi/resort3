use sqparse::ast::{BlockStatement, StatementType};

use crate::{
    get_statement_rep,
    tokens::get_token,
    utils::{get_lead, trim_trailing_newline},
};

pub fn get_block_rep(block: &BlockStatement, depth: usize) -> String {
    let inline_pre = get_lead(depth);
    let statements_pre = get_lead(depth + 1);
    let pre = format!("\n{inline_pre}");

    let lines = block
        .statements
        .iter()
        .map(|statement| {
            let rep = get_statement_rep(&statement.ty, depth + 1);
            let mut lines = rep.split("\n").collect::<Vec<_>>();
            let first_line = lines.get(0);

            // for i in 0..lines.len() {
			// 	let ll = lines[i];
			// 	let l = &mut String::from(ll);
            //     trim_trailing_newline(l);
			// 	lines[i] = l;
            // }

            if rep.find("//") == Some(0)
                || (matches!(first_line, Some(_)) && first_line.unwrap().trim().is_empty())
            {
                rep
            } else {
                format!("{statements_pre}{rep}")
            }
        })
        .collect::<Vec<_>>();

    format!(
        "{pre}{}\n{}{pre}{}",
        get_token(block.open, "{", depth),
        lines.join("\n"),
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
