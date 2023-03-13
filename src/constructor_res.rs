use sqparse::ast::ConstructorDefinitionStatement;

use crate::{function_rep::get_function_def_rep, tokens::get_token};

pub fn get_constructor_def_rep(c: &ConstructorDefinitionStatement, depth: usize) -> String {
    format!(
        "{} {}{}{}{}{}",
        get_token(c.function, "function", depth),
        c.namespaces
            .iter()
            .map(|(namespace, seperator)| format!(
                "{}{}",
                namespace.value,
                get_token(seperator, "::", depth)
            ))
            .collect::<String>(),
        c.last_name.value,
        get_token(c.last_namespace, "::", depth),
		get_token(c.constructor, "constructor", depth),
        get_function_def_rep(&c.definition, depth)
    )
}
