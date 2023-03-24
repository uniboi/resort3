use sqparse::ast::{ForeachIndex, ForeachStatement, Identifier, StatementType, Type};

use crate::{
    get_config,
    rep::{
        expressions::get_expression_rep, statements::get_inline_statement_rep, tokens::get_token,
        type_rep::get_typed_type_rep,
    },
    utils::get_optional_padding,
};

pub fn get_foreach_rep(stm: &ForeachStatement, depth: usize) -> String {
    let gap = get_optional_padding(get_config().foreach_gap);
    let padding = get_optional_padding(get_config().foreach_padding);
    let inline = get_config().foreach_inline;
    format!(
        "{}{gap}{}{padding}{}{} {} {}{padding}{}{}",
        get_token(stm.foreach, "foreach", depth),
        get_token(stm.open, "(", depth),
        match &stm.index {
            Some(idx) => format!("{}, ", get_foreach_index_rep(idx, depth)),
            None => String::new(),
        },
        get_foreach_value_rep(&stm.value_type, &stm.value_name, depth),
        get_token(stm.in_, "in", depth),
        get_expression_rep(&stm.array, depth),
        get_token(stm.close, ")", depth),
        match *stm.body {
            StatementType::Block(_) => {
                get_inline_statement_rep(&stm.body, depth, get_config().foreach_inline_block)
            }
            _ => get_inline_statement_rep(&stm.body, depth, inline),
        },
    )
}

fn get_foreach_index_rep(idx: &ForeachIndex, depth: usize) -> String {
    format!(
        "{}{}",
        match &idx.type_ {
            Some(ty) => format!("{} ", get_typed_type_rep(ty, depth)),
            None => String::new(),
        },
        idx.name.value
    )
}

fn get_foreach_value_rep(ty: &Option<Type>, value: &Identifier, depth: usize) -> String {
    format!(
        "{}{}",
        match ty {
            Some(ty) => format!("{} ", get_typed_type_rep(ty, depth)),
            None => String::new(),
        },
        value.value
    )
}
