use sqparse::ast::{Preprocessable, Slot, TableSlot};

use crate::{
    get_config,
    rep::preprocessed::get_preprocessed_rep,
    utils::{apply_lead_to_lines, get_lead, trim_trailing_newline, rep_includes_single_line_comment},
};

use super::{
    expressions::get_expression_rep,
    function_rep::{get_fragmented_named_function_rep, get_function_def_rep},
    tokens::get_token,
    var_rep::get_var_initializer_rep,
};

pub fn get_table_rep(table: &sqparse::ast::TableExpression, depth: usize) -> String {
    let max_oneliner_items = get_config().table_oneliner_max;
    let mut multiline = table.slots.len() > max_oneliner_items;

    let open = get_token(table.open, "{", depth);
    let close = get_token(table.close, "}", depth);

    if table.slots.is_empty() {
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
        get_multiline_table_rep(table, depth)
    } else {
        let rep = format!(
            "{open} {}{close}",
            table
                .slots
                .iter()
                .map(|slot| match slot {
                    Preprocessable::PREPROCESSED(_) =>
                        todo!("inline preprocessed slots not implemented"),
                    Preprocessable::UNCONDITIONAL(slot) => format!(
                        "{}{} ",
                        get_table_pair_rep(slot, depth),
                        match slot.comma {
                            Some(comma) => get_token(comma, ",", depth),
                            None => String::new(),
                        }
                    ),
                })
                .collect::<String>()
        );
        if rep.find('\n').is_some() || rep_includes_single_line_comment(&rep) {
            return get_multiline_table_rep(table, depth);
        }
        return rep;
    }
}

fn get_multiline_table_rep(table: &sqparse::ast::TableExpression, depth: usize) -> String {
    let lead = get_lead(depth);
    let prop_inset = get_lead(depth + 1);
    let open = get_token(table.open, "{", depth);
    let close = get_token(table.close, "}", depth);

    format!(
        "{open}{}\n{lead}{close}",
        table
            .slots
            .iter()
            .map(|slot| match slot {
                Preprocessable::PREPROCESSED(p) => format!(
                    "\n{prop_inset}{}",
                    get_preprocessed_rep(p, &get_table_pair_rep, depth + 1)
                ),
                Preprocessable::UNCONDITIONAL(slot) => {
                    let mut r = get_table_pair_rep(slot, depth);
                    trim_trailing_newline(&mut r);
                    let rep = apply_lead_to_lines(r.split("\n"), depth + 1);
                    format!(
                        "\n{}{}",
                        rep,
                        match slot.comma {
                            Some(comma) => get_token(comma, ",", depth),
                            None => String::new(),
                        }
                    )
                }
            })
            .collect::<String>(),
    )
}

pub fn get_table_pair_rep(s: &TableSlot, depth: usize) -> String {
    match &s.ty {
        sqparse::ast::TableSlotType::Slot(slot) => get_slot_rep(slot, depth),
        sqparse::ast::TableSlotType::JsonProperty {
            name,
            name_token,
            colon,
            value,
        } => format!(
            "\"{}\" {} {}",
            get_token(name_token, name, depth),
            get_token(colon, ":", depth),
            get_expression_rep(value, depth)
        ),
    }
}

pub fn get_slot_rep(s: &Slot, depth: usize) -> String {
    match &s {
        sqparse::ast::Slot::Property { name, initializer } => format!(
            "{}{}",
            get_token(name.token, name.value, depth),
            get_var_initializer_rep(initializer, depth)
        ),
        sqparse::ast::Slot::ComputedProperty {
            open,
            name,
            close,
            initializer,
        } => format!(
            "{} {} {}{}",
            get_token(open, "[", depth),
            get_expression_rep(name, depth),
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
