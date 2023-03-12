use sqparse::ast::{Preprocessable, StructDefinition, StructDefinitionStatement, StructProperty};

use crate::{
    preprocessed::get_preprocessed_rep, type_rep::get_typed_type_rep, utils::get_lead,
    var_rep::get_var_initializer_rep,
};

pub fn get_struct_definition_rep(p: &StructDefinitionStatement, depth: usize) -> String {
    let lead = get_lead(depth);
    format!(
        "struct {}\n{lead}{{{}\n{lead}}}",
        p.name.value,
        get_struct_def_rep(&p.definition, depth + 1)
    )
}

pub fn get_anon_struct_definition_rep(t: &StructDefinition, depth: usize) -> String {
    let lead = get_lead(depth);
    format!("struct {{{}\n{lead}}}", get_struct_def_rep(t, depth + 1))
}

fn get_struct_def_rep(def: &StructDefinition, depth: usize) -> String {
	let lead = get_lead(depth);
    def.properties
        .iter()
        .map(|property| match property {
            Preprocessable::PREPROCESSED(preprocessed) => format!(
                "\n{lead}{}",
                get_preprocessed_rep(
                    preprocessed,
                    &|property, depth| get_struct_property_rep(property, depth),
                    depth,
                )
            ),
            Preprocessable::UNCONDITIONAL(property) => {
                format!("\n{lead}{}", get_struct_property_rep(property, depth))
            }
        })
        .collect::<String>()
}

fn get_struct_property_rep(property: &StructProperty, depth: usize) -> String {
    format!(
        "{} {}{}",
        get_typed_type_rep(&property.type_, depth),
        property.name.value,
        match &property.initializer {
            Some(initializer) => get_var_initializer_rep(initializer, depth),
            None => String::new(),
        }
    )
}
