use sqparse::ast::{
    ClassDefinition, ClassDefinitionStatement, ClassExpression, ClassExtends, ClassMember,
};

use crate::{get_expression_rep, table_rep::get_slot_rep, utils::get_lead};

pub fn get_class_expression_rep(p: &ClassExpression, depth: usize) -> String {
    format!("class {}", get_class_def_rep(&p.definition, depth))
}

pub fn get_class_statement_rep(p: &ClassDefinitionStatement, depth: usize) -> String {
    format!(
        "class {} {}",
        get_expression_rep(&*p.name, depth),
        get_class_def_rep(&p.definition, depth)
    )
}

fn get_class_def_rep(def: &ClassDefinition, depth: usize) -> String {
    let lead = get_lead(depth);
    format!(
        "{}\n{lead}{{{}{lead}}}",
        match &def.extends {
            Some(ext) => get_class_extend_rep(ext, depth),
            None => String::new(),
        },
        get_class_members_rep(&def.members, depth + 1)
    )
}

// TODO: Add class attributes
fn get_class_members_rep(members: &Vec<ClassMember>, depth: usize) -> String {
    let lead = get_lead(depth);
    format!(
        "\n{}",
        members
            .iter()
            .map(|member| format!(
                "{lead}{}{}\n",
                match &member.static_ {
                    Some(_) => "static ",
                    None => "",
                },
                get_slot_rep(&member.slot, depth)
            ))
            .collect::<String>()
    )
}

fn get_class_extend_rep(ext: &ClassExtends, depth: usize) -> String {
    format!("extends {}", get_expression_rep(&*ext.name, depth))
}
