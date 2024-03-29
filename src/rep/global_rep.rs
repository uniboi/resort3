use sqparse::ast::GlobalStatement;

use crate::rep::{
    class_rep::get_class_statement_rep,
    enum_rep::get_enum_rep,
    struct_rep::get_struct_definition_rep,
    tokens::get_token,
    type_rep::get_typedef_rep,
    var_rep::{get_const_rep, get_var_definition_list_rep, get_var_initializer_rep},
};

pub fn get_global_rep(statement: &GlobalStatement, depth: usize) -> String {
    let global_rep = match &statement.definition {
        sqparse::ast::GlobalDefinition::Function { function, name } => {
            format!(
                "{} {}",
                get_token(function, "function", depth),
                get_token(name.token, name.value, depth)
            )
        }
        sqparse::ast::GlobalDefinition::UntypedVar { name, initializer } => {
            const EXPLICIT_TYPES: bool = false; // TODO: read from config
            let ty = if EXPLICIT_TYPES {
                "TODO_EXPLICIT_TYPES "
            } else {
                ""
            };
            format!(
                "{ty}{}{}",
                name.value,
                get_var_initializer_rep(initializer, depth)
            )
        }
        sqparse::ast::GlobalDefinition::TypedVar(p) => get_var_definition_list_rep(p, depth),
        sqparse::ast::GlobalDefinition::Const(p) => get_const_rep(p, depth),
        sqparse::ast::GlobalDefinition::Enum(p) => get_enum_rep(p, depth),
        sqparse::ast::GlobalDefinition::Class(p) => get_class_statement_rep(p, depth),
        sqparse::ast::GlobalDefinition::Struct(p) => get_struct_definition_rep(p, depth),
        sqparse::ast::GlobalDefinition::Type(p) => get_typedef_rep(p, depth),
    };
    format!(
        "{} {global_rep}",
        get_token(statement.global, "global", depth)
    )
}
