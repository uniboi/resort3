use sqparse::ast::{ClassDefinition, ClassDefinitionStatement, ClassExpression, ClassExtends};

use crate::get_expression_rep;

pub fn get_class_expression_rep(p: &ClassExpression, depth: usize) -> String {
    format!("class {}", get_class_def_rep(&p.definition, depth))
}

fn get_class_def_rep(def: &ClassDefinition, depth: usize) -> String {
    format!(
        "{}",
        match &def.extends {
            Some(ext) => get_class_extend_rep(ext, depth),
            None => String::new(),
        }
    )
}

fn get_class_extend_rep(ext: &ClassExtends, depth: usize) -> String {
    format!("extends {}", get_expression_rep(&*ext.name, depth))
}
