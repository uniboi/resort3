use sqparse::ast::{
    ClassDefinition, ClassDefinitionStatement, ClassExpression, ClassExtends, ClassMember,
};

use crate::{rep::{tokens::get_token, expressions::get_expression_rep, table_rep::get_slot_rep}, utils::get_lead};


pub fn get_class_expression_rep(p: &ClassExpression, depth: usize) -> String {
    format!(
        "{} {}",
        get_token(p.class, "class", depth),
        get_class_def_rep(&p.definition, depth)
    )
}

pub fn get_class_statement_rep(p: &ClassDefinitionStatement, depth: usize) -> String {
    format!(
        "{} {} {}",
        get_token(p.class, "class", depth),
        get_expression_rep(&p.name, depth),
        get_class_def_rep(&p.definition, depth)
    )
}

fn get_class_def_rep(def: &ClassDefinition, depth: usize) -> String {
    let lead = get_lead(depth);
    format!(
        "{}\n{lead}{}{}{lead}{}",
        match &def.extends {
            Some(ext) => get_class_extend_rep(ext, depth),
            None => String::new(),
        },
        get_token(def.open, "}", depth),
        get_class_members_rep(&def.members, depth + 1),
        get_token(def.close, "}", depth)
    )
}

// TODO: Add class attributes
fn get_class_members_rep(members: &[ClassMember], depth: usize) -> String {
    let lead = get_lead(depth);
    format!(
        "\n{}",
        members
            .iter()
            .map(|member| format!(
                "{lead}{}{}\n",
                match &member.static_ {
                    Some(token) => format!("{} ", get_token(token, "static", depth)),
                    None => String::new(),
                },
                get_slot_rep(&member.slot, depth)
            ))
            .collect::<String>()
    )
}

fn get_class_extend_rep(ext: &ClassExtends, depth: usize) -> String {
    format!(
        "{} {}",
        get_token(ext.extends, "extends", depth),
        get_expression_rep(&ext.name, depth)
    )
}
