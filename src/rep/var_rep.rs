use sqparse::ast::{
    ConstDefinitionStatement, SeparatedListTrailing1, VarDefinition, VarDefinitionStatement,
    VarInitializer,
};

use crate::rep::{
    expressions::get_expression_rep, tokens::get_token, type_rep::get_typed_type_rep,
};

pub fn get_const_rep(statement: &ConstDefinitionStatement, depth: usize) -> String {
    let type_rep = match &statement.const_type {
        Some(ty) => format!("{} ", get_typed_type_rep(ty, depth)),
        None => String::new(),
    };
    format!(
        "{} {type_rep}{}{}",
        get_token(statement.const_, "const", depth),
        statement.name.value,
        get_var_initializer_rep(&statement.initializer, depth)
    )
}

pub fn get_var_definition_list_rep(statement: &VarDefinitionStatement, depth: usize) -> String {
    format!(
        "{} {}{}",
        get_typed_type_rep(&statement.type_, depth),
        get_definition_list_rep(&statement.definitions, depth),
        match &statement.definitions.last_item.initializer {
            Some(initializer) => get_var_initializer_rep(initializer, depth),
            None => String::new(),
        }
    )
}

fn get_definition_list_rep(list: &SeparatedListTrailing1<VarDefinition>, depth: usize) -> String {
    format!(
        "{}{}",
        list.items
            .iter()
            .map(|(v, comma)| format!("{}{} ", v.name.value, get_token(comma, ",", depth)))
            .collect::<String>(),
        list.last_item.name.value
    )
}

// pub fn get_var_definition_rep(statement: &VarDefinition, depth: usize) -> String {
//     format!(
//         "{}{}",
//         statement.name.value,
//         match &statement.initializer {
//             Some(initializer) => get_var_initializer_rep(initializer, depth),
//             None => String::new(),
//         }
//     )
// }

pub fn get_var_initializer_rep(statement: &VarInitializer, depth: usize) -> String {
    format!(
        " {} {}",
        get_token(statement.assign, "=", depth),
        get_expression_rep(&*statement.value, depth)
    )
}
