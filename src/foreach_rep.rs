use sqparse::ast::{ForeachIndex, ForeachStatement};

use crate::{block_rep::get_inset_statement_rep, get_expression_rep, type_rep::get_typed_type_rep};

pub fn get_foreach_rep(stm: &ForeachStatement, depth: usize) -> String {
    format!(
        "foreach( {}{} {} in {} ){}",
        match &stm.index {
            Some(idx) => format!("{}, ", get_foreach_index_rep(idx, depth)),
            None => String::new(),
        },
        match &stm.value_type {
            Some(ty) => get_typed_type_rep(ty, depth),
            None => String::new(),
        },
        stm.value_name.value,
        get_expression_rep(&*stm.array, depth),
        get_inset_statement_rep(&*stm.body, depth)
    )
}

fn get_foreach_index_rep(idx: &ForeachIndex, depth: usize) -> String {
    format!(
        "{} {}",
        match &idx.type_ {
            Some(ty) => get_typed_type_rep(ty, depth),
            None => String::new(),
        },
        idx.name.value
    )
}
