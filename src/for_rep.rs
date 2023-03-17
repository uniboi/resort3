use sqparse::ast::ForStatement;

use crate::{
    get_expression_rep, get_statement_rep, tokens::get_token, utils::get_lead,
    var_rep::get_var_definition_list_rep,
};

pub fn get_for_rep(stm: &ForStatement, depth: usize) -> String {
    format!(
        "{}{} {}{} {}{} {} {}{}",
        get_token(stm.for_, "for", depth),
        get_token(stm.open, "(", depth),
        match &stm.initializer {
            Some(initializer) => match initializer {
                sqparse::ast::ForDefinition::Expression(initializer) =>
                    get_expression_rep(initializer, depth),
                sqparse::ast::ForDefinition::Definition(initializer) =>
                    get_var_definition_list_rep(initializer, depth),
            },
            None => String::new(),
        },
        get_token(stm.semicolon_1, ";", depth),
        match &stm.condition {
            Some(condition) => get_expression_rep(condition, depth),
            None => String::new(),
        },
        get_token(stm.semicolon_2, ";", depth),
        match &stm.increment {
            Some(increment) => get_expression_rep(increment, depth),
            None => String::new(),
        },
        get_token(stm.close, ")", depth),
        match &*stm.body {
            sqparse::ast::StatementType::Block(_) => get_statement_rep(&*stm.body, depth),
            _ => format!(
                "\n{}{}",
                get_lead(depth + 1),
                get_statement_rep(&*stm.body, depth)
            ),
        }
    )
}
