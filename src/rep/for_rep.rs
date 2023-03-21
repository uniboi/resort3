use sqparse::ast::ForStatement;

use crate::{
    get_config,
    rep::{
        expressions::get_expression_rep, statements::get_statement_rep, tokens::get_token,
        var_rep::get_var_definition_list_rep,
    },
    utils::{get_lead, get_optional_padding},
};

pub fn get_for_rep(stm: &ForStatement, depth: usize) -> String {
    let gap = get_optional_padding(get_config().for_gap);
    let padding = get_optional_padding(get_config().for_padding);
    let inline = get_config().for_inline;

    let head_rep = if let (None, None, None) = (&stm.initializer, &stm.condition, &stm.initializer)
    {
        format!(
            "{}{}",
            get_token(stm.semicolon_1, ";", depth),
            get_token(stm.semicolon_2, ";", depth),
        )
    } else {
        format!(
            "{padding}{}{} {}{} {}{padding}",
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
        )
    };

    format!(
        "{}{gap}{}{head_rep}{}{}",
        get_token(stm.for_, "for", depth),
        get_token(stm.open, "(", depth),
        get_token(stm.close, ")", depth),
        match &*stm.body {
            sqparse::ast::StatementType::Block(_) =>
                if get_config().for_inline_block {
                    format!(" {}", get_statement_rep(&*stm.body, depth))
                } else {
                    format!(
                        "\n{}{}",
                        get_lead(depth),
                        get_statement_rep(&*stm.body, depth)
                    )
                },
            _ =>
                if inline {
                    format!(" {}", get_statement_rep(&*stm.body, depth))
                } else {
                    format!(
                        "\n{}{}",
                        get_lead(depth + 1),
                        get_statement_rep(&*stm.body, depth)
                    )
                },
        }
    )
}
