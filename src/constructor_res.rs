use sqparse::ast::ConstructorDefinitionStatement;

use crate::function_rep::get_function_def_rep;

pub fn get_constructor_def_rep(c: &ConstructorDefinitionStatement, depth: usize) -> String {
    format!("function {}::constructor{}", c.last_name.value, get_function_def_rep(&c.definition, depth))
}
