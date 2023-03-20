use sqparse::ast::{EnumDefinitionStatement, EnumEntry};

use crate::{
    rep::{tokens::get_token, var_rep::get_var_initializer_rep},
    utils::get_lead,
};

pub fn get_enum_rep(p: &EnumDefinitionStatement, depth: usize) -> String {
    let lead = get_lead(depth);
    format!(
        "{} {}\n{}{}\n{}\n{}{}",
        get_token(p.enum_, "enum", depth),
        p.name.value,
        lead,
        get_token(p.open, "{", depth),
        p.entries
            .iter()
            .map(|entry| get_enum_entry_rep(entry, depth + 1))
            .collect::<Vec<_>>()
            .join(",\n"),
        lead,
        get_token(p.close, "}", depth),
    )
}

fn get_enum_entry_rep(entry: &EnumEntry, depth: usize) -> String {
    let lead = get_lead(depth);
    format!(
        "{lead}{}{}",
        entry.name.value,
        match &entry.initializer {
            Some(initializer) => get_var_initializer_rep(initializer, depth),
            None => String::new(),
        }
    )
}
