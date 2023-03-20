use sqparse::ast::{ForeachIndex, ForeachStatement, Identifier, Type};

use crate::rep::{
    block_rep::get_inset_statement_rep, expressions::get_expression_rep, tokens::get_token,
    type_rep::get_typed_type_rep,
};

pub fn get_foreach_rep(stm: &ForeachStatement, depth: usize) -> String {
    format!(
        "{}{}{}{} {} {} {}{}",
        get_token(stm.foreach, "foreach", depth),
        get_token(stm.open, "(", depth),
        match &stm.index {
            Some(idx) => format!("{},", get_foreach_index_rep(idx, depth)),
            None => String::new(),
        },
        get_foreach_value_rep(&stm.value_type, &stm.value_name, depth),
        get_token(stm.in_, "in", depth),
        get_expression_rep(&*stm.array, depth),
        get_token(stm.close, ")", depth),
        get_inset_statement_rep(&*stm.body, depth)
    )
}

fn get_foreach_index_rep(idx: &ForeachIndex, depth: usize) -> String {
    format!(
        " {}{}",
        match &idx.type_ {
            Some(ty) => format!("{} ", get_typed_type_rep(ty, depth)),
            None => String::new(),
        },
        idx.name.value
    )
}

fn get_foreach_value_rep(ty: &Option<Type>, value: &Identifier, depth: usize) -> String {
    format!(
        " {}{}",
        match ty {
            Some(ty) => format!("{} ", get_typed_type_rep(ty, depth)),
            None => String::new(),
        },
        value.value
    )
}
