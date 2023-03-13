use sqparse::ast::{Preprocessable, Slot, TableSlot};

use crate::{
    function_rep::{get_fragmented_named_function_rep, get_function_def_rep},
    get_expression_rep,
    preprocessed::get_preprocessed_rep,
    tokens::get_token,
    utils::get_lead,
    var_rep::get_var_initializer_rep,
};

pub fn get_table_rep(table: &sqparse::ast::TableExpression, depth: usize) -> String {
    let prop_inset = get_lead(depth + 1);
    let max_oneliner_items = 3; // TODO: read from config
    let mut multiline = table.slots.len() > max_oneliner_items;

    let open = get_token(table.open, "{", depth);
    let close = get_token(table.close, "}", depth);

    if table.slots.len() == 0 {
        return format!("{open}{close}");
    }

    if !multiline {
        for slot in &table.slots {
            if matches!(slot, Preprocessable::PREPROCESSED(_)) {
                multiline = true;
                break;
            }
        }
    }

    if multiline {
        format!(
            "{open}\n{}{}\n{}{close}",
            prop_inset,
            table
                .slots
                .iter()
                .map(|slot| match slot {
                    Preprocessable::PREPROCESSED(p) => format!(
                        "\n{prop_inset}{}",
                        get_preprocessed_rep(p, &get_table_pair_rep, depth + 1)
                    ),
                    Preprocessable::UNCONDITIONAL(slot) => format!(
                        "\n{prop_inset}{}{}",
                        get_table_pair_rep(slot, depth),
                        match slot.comma {
                            Some(comma) => get_token(comma, ",", depth),
                            None => String::new(),
                        }
                    ),
                })
                .collect::<String>(),
            get_lead(depth)
        )
    } else {
        format!(
            "{open} {} {close}",
            table
                .slots
                .iter()
                .map(|slot| match slot {
                    Preprocessable::PREPROCESSED(_) =>
                        todo!("inline preprocessed slots not implemented"),
                    Preprocessable::UNCONDITIONAL(slot) => format!(
                        "{}{}",
                        get_table_pair_rep(slot, depth),
                        match slot.comma {
                            Some(comma) => get_token(comma, ",", depth),
                            None => String::new(),
                        }
                    ),
                })
                .collect::<String>()
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
                name_token, // TODO: unused?
                colon,
                value,
            } => format!(
                "\"{name}\" {} {}",
                get_token(colon, ":", depth),
                get_expression_rep(&*value, depth)
            ),
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
            open,
            name,
            close,
            initializer,
        } => format!(
            "{}{}{}{}",
            get_token(open, "[", depth),
            get_expression_rep(&*name, depth),
            get_token(close, "]", depth),
            get_var_initializer_rep(initializer, depth)
        ),
        sqparse::ast::Slot::Constructor {
            function,
            constructor,
            definition,
        } => format!(
            "{}{}{}",
            match function {
                Some(function) => format!("{} ", get_token(function, "function", depth)),
                None => String::new(),
            },
            get_token(constructor, "constructor", depth),
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
