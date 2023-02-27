use sqparse::ast::ForStatement;

use crate::{get_expression_rep, var_rep::get_var_definition_list_rep, get_statement_rep};

pub fn get_for_rep(stm: &ForStatement, depth: usize) -> String {
    format!(
        "for( {}; {}; {} ){}",
        match &stm.initializer {
            Some(initializer) => match initializer {
                sqparse::ast::ForDefinition::Expression(initializer) =>
                    get_expression_rep(initializer, depth),
                sqparse::ast::ForDefinition::Definition(initializer) =>
                    get_var_definition_list_rep(initializer, depth),
            },
            None => String::new(),
        },
        match &stm.condition {
            Some(condition) => get_expression_rep(condition, depth),
            None => String::new(),
        },
        match &stm.increment {
            Some(increment) => get_expression_rep(increment, depth),
            None => String::new(),
        },
		get_statement_rep(&*stm.body, depth)
    )
}
