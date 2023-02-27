use sqparse::ast::{EnumDefinitionStatement, EnumEntry};

use crate::{utils::get_lead, var_rep::get_var_initializer_rep};

pub fn get_enum_rep(p: &EnumDefinitionStatement, depth: usize) -> String {
    let lead = get_lead(depth);
    format!(
        "enum {}\n{}{{\n{}\n{}}}",
        p.name.value,
        lead,
        p.entries
            .iter()
            .map(|entry| get_enum_entry_rep(entry, depth + 1))
            .collect::<Vec<_>>()
            .join(",\n"),
        lead
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
