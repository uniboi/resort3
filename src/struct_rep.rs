use sqparse::ast::{
    Preprocessable, StructDefinition, StructDefinitionStatement, StructProperty, StructType,
};

use crate::{
    preprocessed::get_preprocessed_rep, tokens::get_token, type_rep::get_typed_type_rep,
    utils::get_lead, var_rep::get_var_initializer_rep,
};

pub fn get_struct_definition_rep(p: &StructDefinitionStatement, depth: usize) -> String {
    let lead = get_lead(depth);
    format!(
        "{} {}\n{lead}{}{}\n{lead}{}",
        get_token(p.struct_, "struct"),
        p.name.value,
        get_token(p.definition.open, "("),
        get_struct_def_rep(&p.definition, depth + 1),
        get_token(p.definition.close, ")"),
    )
}

pub fn get_anon_struct_definition_rep(p: &StructType, depth: usize) -> String {
    let lead = get_lead(depth);
    format!(
        "{} {}{}\n{lead}{}",
        get_token(p.struct_, "struct"),
        get_token(p.definition.open, "("),
        get_struct_def_rep(&p.definition, depth + 1),
        get_token(p.definition.close, ")"),
    )
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
