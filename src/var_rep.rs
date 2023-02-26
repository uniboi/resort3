use sqparse::ast::{
    ConstDefinitionStatement, SeparatedListTrailing1, VarDefinition, VarDefinitionStatement,
    VarInitializer,
};

use crate::{get_expression_rep, type_rep::get_typed_type_rep};

pub fn get_const_rep(statement: &ConstDefinitionStatement) -> String {
    let type_rep = match &statement.const_type {
        Some(ty) => format!("{} ", get_typed_type_rep(ty)),
        None => String::new(),
    };
    format!(
        "const {type_rep}{}{}",
        statement.name.value,
        get_var_initializer_rep(&statement.initializer)
    )
}

pub fn get_var_definition_list_rep(statement: &VarDefinitionStatement) -> String {
    format!(
        "{} {}",
        get_typed_type_rep(&statement.type_),
        get_definition_list_rep(&statement.definitions)
    )
}

fn get_definition_list_rep(list: &SeparatedListTrailing1<VarDefinition>) -> String {
    format!(
        "{}{}{}",
        list.items
            .iter()
            .map(|(v, _)| v.name.value)
            .collect::<Vec<_>>()
            .join(", "),
        if list.items.len() > 0 { ", " } else { "" },
        list.last_item.name.value
    )
}

pub fn get_var_definition_rep(statement: &VarDefinition) -> String {
    format!(
        "{}{}",
        statement.name.value,
        match &statement.initializer {
            Some(initializer) => get_var_initializer_rep(initializer),
            None => String::new(),
        }
    )
}

pub fn get_var_initializer_rep(statement: &VarInitializer) -> String {
    format!(" = {}", get_expression_rep(&*statement.value))
}
