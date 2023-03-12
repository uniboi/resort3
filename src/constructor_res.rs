use sqparse::ast::ConstructorDefinitionStatement;

use crate::{function_rep::get_function_def_rep, tokens::get_token};

pub fn get_constructor_def_rep(c: &ConstructorDefinitionStatement, depth: usize) -> String {
    format!(
        "{} {}{}{}{}{}",
        get_token(c.function, "function"),
        c.namespaces
            .iter()
            .map(|(namespace, seperator)| format!(
                "{}{}",
                namespace.value,
                get_token(seperator, "::")
            ))
            .collect::<String>(),
        c.last_name.value,
        get_token(c.last_namespace, "::"),
		get_token(c.constructor, "constructor"),
        get_function_def_rep(&c.definition, depth)
    )
}
