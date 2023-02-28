use sqparse::ast::{Slot, TableSlot};

use crate::{
    function_rep::{get_function_def_rep, get_fragmented_named_function_rep}, get_expression_rep, utils::get_lead,
    var_rep::get_var_initializer_rep,
};

pub fn get_table_rep(table: &sqparse::ast::TableExpression, depth: usize) -> String {
    let prop_inset = get_lead(depth + 1);
    let sep = format!(",\n{prop_inset}");
    let max_oneliner_items = 3; // TODO: read from config

    if table.slots.len() == 0 {
        return format!("{{}}");
    }

    if table.slots.len() > max_oneliner_items {
        format!(
            "{{{}\n{}{}\n{}}}",
            prop_inset,
            prop_inset,
            table
                .slots
                .iter()
                .map(|slot| get_table_pair_rep(slot, depth))
                .collect::<Vec<_>>()
                .join(&sep),
            get_lead(depth)
        )
    } else {
        format!(
            "{{ {} }}",
            table
                .slots
                .iter()
                .map(|slot| get_table_pair_rep(slot, depth))
                .collect::<Vec<_>>()
                .join(", ")
        )
    }
}

pub fn get_table_pair_rep(s: &TableSlot, depth: usize) -> String {
    format!(
        "{}",
        match &s.ty {
            sqparse::ast::TableSlotType::Slot(slot) => get_slot_rep(slot, depth),
            sqparse::ast::TableSlotType::JsonProperty {
                name,
                name_token: _,
                colon: _,
                value,
            } => format!("\"{name}\" = {}", get_expression_rep(&*value, depth)),
        }
    )
}

pub fn get_slot_rep(s: &Slot, depth: usize) -> String {
    match &s {
        sqparse::ast::Slot::Property { name, initializer } => format!(
            "{}{}",
            name.value,
            get_var_initializer_rep(initializer, depth)
        ),
        sqparse::ast::Slot::ComputedProperty {
            open: _,
            name,
            close: _,
            initializer,
        } => format!(
            "[{}]{}",
            get_expression_rep(&*name, depth),
            get_var_initializer_rep(initializer, depth)
        ),
        sqparse::ast::Slot::Constructor {
            function,
            constructor: _,
            definition,
        } => format!(
            "{}constructor{}",
            match function {
                Some(_) => "function ",
                None => "",
            },
            get_function_def_rep(definition, depth)
        ),
        sqparse::ast::Slot::Function {
            return_type,
            function,
            name,
            definition,
        } => get_fragmented_named_function_rep(return_type, function, name, definition, depth),
    }
}
